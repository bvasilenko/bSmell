use assert_cmd::Command;
use bsmell::SmellCategory;
use predicates::prelude::*;

const PLACEHOLDER_DIRECTIVE_HEADER: &str = "[bsmell placeholder directive - pre-corpus output]";
const ACTION_PREFIX: &str = "ACTION: This invocation reached bsmell";
const EXIT_CODE_FOOTER: &str = "Exit code carries the verdict-class signal.";
const PUBLIC_INVOCATION_SURFACE_LINE: &str = "Invocation surface: cli.";
const INTERNAL_SURFACE_TOKENS: [&str; 6] = ["L2a", "L2b", "L2c", "l2a", "l2b", "l2c"];
const SCAN_FINDING_EXIT_CODE: i32 = 1;

#[derive(Debug, Clone, Copy)]
struct ScanCase<'a> {
    args: &'a [&'a str],
    expected_input: &'a str,
}

impl ScanCase<'_> {
    fn assert(self) {
        let stdout = scan_stdout(self.args);

        assert_scan_directive(&stdout);
        assert!(stdout.contains(self.expected_input));
    }
}

fn bsmell_command() -> Command {
    Command::cargo_bin("bsmell").expect("binary exists")
}

fn command_stdout_with_code(args: &[&str], code: i32) -> String {
    let output = bsmell_command()
        .args(args)
        .assert()
        .code(code)
        .get_output()
        .clone();

    String::from_utf8(output.stdout).expect("stdout is utf8")
}

fn scan_stdout(args: &[&str]) -> String {
    command_stdout_with_code(args, SCAN_FINDING_EXIT_CODE)
}

fn successful_stdout(args: &[&str]) -> String {
    command_stdout_with_code(args, 0)
}

fn assert_usage_failure(args: &[&str], stderr_fragment: &str) {
    bsmell_command()
        .args(args)
        .assert()
        .code(64)
        .stderr(predicate::str::contains(stderr_fragment));
}

fn assert_scan_directive(stdout: &str) {
    assert!(stdout.contains(PLACEHOLDER_DIRECTIVE_HEADER));
    assert!(stdout.contains("Parsed input: session="));
    assert!(stdout.contains("diff="));
    assert!(stdout.contains("Routing key: SmellCategory::"));
    assert!(stdout.contains(" placeholder route."));
    assert!(stdout.contains(PUBLIC_INVOCATION_SURFACE_LINE));
    assert!(stdout.contains("Evidence-state: SmellDetected."));
    assert!(stdout.contains(ACTION_PREFIX));
    assert!(stdout.contains("deflection pattern "));
    assert!(stdout.contains("re-anchoring on the underlying task"));
    assert!(stdout.contains(EXIT_CODE_FOOTER));
    assert!(!stdout.contains("detection behavior is deferred"));
    assert!(!stdout.contains("detection=not-run"));
    assert!(!stdout.contains("\"detection\":\"not-run\""));
    assert_no_internal_surface_tokens(stdout);
}

fn assert_no_internal_surface_tokens(stdout: &str) {
    for token in INTERNAL_SURFACE_TOKENS {
        assert!(!stdout.contains(token), "stdout leaked {token}: {stdout}");
    }
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
fn scan_emits_placeholder_directive_and_finding_exit_code() {
    ScanCase {
        args: &["scan", "--reason", "review requested"],
        expected_input: "session=<none>, diff=<none>",
    }
    .assert();
}

#[test]
fn scan_directive_reports_every_supported_input_combination() {
    for scan_case in [
        ScanCase {
            args: &["scan"],
            expected_input: "session=<none>, diff=<none>",
        },
        ScanCase {
            args: &["scan", "--session", "./Cargo.toml"],
            expected_input: "session=./Cargo.toml, diff=<none>",
        },
        ScanCase {
            args: &["scan", "--manifest", "manifest.json"],
            expected_input: "session=<none>, diff=<none>",
        },
        ScanCase {
            args: &["scan", "--diff", "change.diff"],
            expected_input: "session=<none>, diff=change.diff",
        },
        ScanCase {
            args: &["scan", "--session", "3", "--diff", "change.diff"],
            expected_input: "session=3, diff=change.diff",
        },
    ] {
        scan_case.assert();
    }
}

#[test]
fn scan_quiet_and_json_flags_keep_directive_stdout() {
    for scan_case in [
        ScanCase {
            args: &["scan", "--quiet", "--reason", "review requested"],
            expected_input: "session=<none>, diff=<none>",
        },
        ScanCase {
            args: &["scan", "--json", "--reason", "review requested"],
            expected_input: "session=<none>, diff=<none>",
        },
        ScanCase {
            args: &["scan", "--quiet", "--json", "--reason", "review requested"],
            expected_input: "session=<none>, diff=<none>",
        },
    ] {
        scan_case.assert();
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
fn scan_accepts_every_non_blank_reason_shape() {
    for reason in [
        "review requested",
        " review requested ",
        "review\trequested",
    ] {
        ScanCase {
            args: &["scan", "--reason", reason],
            expected_input: "session=<none>, diff=<none>",
        }
        .assert();
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
