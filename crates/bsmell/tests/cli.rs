use assert_cmd::Command;
use bsmell::SmellCategory;
use predicates::prelude::*;
use serde_json::Value;
use std::collections::BTreeSet;

fn bsmell_command() -> Command {
    Command::cargo_bin("bsmell").expect("binary exists")
}

fn successful_stdout(args: &[&str]) -> String {
    let output = bsmell_command()
        .args(args)
        .assert()
        .success()
        .get_output()
        .clone();

    String::from_utf8(output.stdout).expect("stdout is utf8")
}

fn scan_json(args: &[&str]) -> Value {
    let stdout = successful_stdout(args);

    serde_json::from_str(stdout.trim()).expect("scan json output is valid json")
}

fn assert_object_keys(value: &Value, expected_keys: &[&str]) {
    let actual_keys = value
        .as_object()
        .expect("value is a json object")
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let expected_keys = expected_keys.iter().copied().collect::<BTreeSet<_>>();

    assert_eq!(expected_keys, actual_keys);
}

fn assert_usage_failure(args: &[&str], stderr_fragment: &str) {
    bsmell_command()
        .args(args)
        .assert()
        .code(64)
        .stderr(predicate::str::contains(stderr_fragment));
}

fn deferred_command_output(command_name: &str) -> String {
    format!("bsmell {command_name} placeholder: behavior is deferred.\n")
}

#[test]
fn help_exits_successfully() {
    bsmell_command()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("CLI deflection-pattern detector"));
}

#[test]
fn categories_exit_successfully_and_print_exact_closed_set() {
    let stdout = successful_stdout(&["categories"]);
    let actual = stdout.lines().collect::<Vec<_>>();
    let expected = SmellCategory::ALL
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    assert_eq!(expected, actual);
}

#[test]
fn placeholder_commands_exit_successfully_with_stable_output() {
    for command_name in ["update", "init", "tail", "explain"] {
        let stdout = successful_stdout(&[command_name]);

        assert_eq!(deferred_command_output(command_name), stdout);
    }
}

#[test]
fn scan_text_placeholder_declares_detection_deferred_and_not_run() {
    let stdout = successful_stdout(&["scan", "--reason", "review requested"]);

    assert_eq!(
        "bsmell scan placeholder: detection behavior is deferred; detection=not-run.\n",
        stdout
    );
}

#[test]
fn scan_json_placeholder_has_stable_schema_and_not_run_verdict() {
    let json = scan_json(&["scan", "--json", "--reason", "review requested"]);

    assert_object_keys(
        &json,
        &["detection", "inputs", "reason", "routing_key", "status"],
    );
    assert_object_keys(&json["inputs"], &["diff", "session"]);
    assert_eq!(json["status"], "placeholder");
    assert_eq!(json["routing_key"], "bsmell");
    assert_eq!(json["detection"], "not-run");
    assert_eq!(json["reason"], "detection behavior is deferred");
    assert_eq!(json["inputs"]["session"], false);
    assert_eq!(json["inputs"]["diff"], false);
}

#[test]
fn scan_quiet_mode_suppresses_placeholder_output_for_every_output_format() {
    for args in [
        &["scan", "--quiet", "--reason", "review requested"][..],
        &["scan", "--quiet", "--json", "--reason", "review requested"][..],
    ] {
        bsmell_command()
            .args(args)
            .assert()
            .success()
            .stdout(predicate::str::is_empty());
    }
}

#[test]
fn scan_json_reports_input_presence_for_every_supported_input_combination() {
    for (args, expected_session, expected_diff) in [
        (&["scan", "--json"][..], false, false),
        (&["scan", "--json", "--session", "3"][..], true, false),
        (
            &["scan", "--json", "--manifest", "manifest.json"][..],
            false,
            false,
        ),
        (
            &["scan", "--json", "--diff", "change.diff"][..],
            false,
            true,
        ),
        (
            &["scan", "--json", "--session", "3", "--diff", "change.diff"][..],
            true,
            true,
        ),
    ] {
        let json = scan_json(args);

        assert_eq!(json["detection"], "not-run");
        assert_eq!(json["inputs"]["session"], expected_session);
        assert_eq!(json["inputs"]["diff"], expected_diff);
    }
}

#[test]
fn scan_rejects_blank_reason() {
    for blank_reason in ["", " ", "   ", "\t", "\n"] {
        bsmell_command()
            .args(["scan", "--reason", blank_reason])
            .assert()
            .code(64)
            .stderr(predicate::str::contains("reason must not be empty"));
    }
}

#[test]
fn scan_accepts_every_non_blank_reason_shape_without_running_detection() {
    for reason in [
        "review requested",
        " review requested ",
        "review\trequested",
    ] {
        let json = scan_json(&["scan", "--json", "--reason", reason]);

        assert_eq!(json["detection"], "not-run");
        assert_eq!(json["reason"], "detection behavior is deferred");
    }
}

#[test]
fn unknown_command_uses_cli_usage_failure() {
    assert_usage_failure(&["unknown"], "unrecognized subcommand");
}

#[test]
fn malformed_flag_shape_uses_cli_usage_failure() {
    for (args, stderr_fragment) in [
        (&["scan", "--reason"][..], "a value is required"),
        (&["scan", "--unknown"][..], "unexpected argument"),
        (&["scan", "--json=false"][..], "unexpected value"),
    ] {
        assert_usage_failure(args, stderr_fragment);
    }
}
