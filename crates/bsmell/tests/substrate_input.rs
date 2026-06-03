use bsmell::{BsmellError, substrate_input::SubstrateInput};
use std::path::PathBuf;

#[test]
fn substrate_input_accepts_every_supported_presence_combination() {
    for (session, diff) in [
        (None, None),
        (Some(PathBuf::from("3")), None),
        (None, Some(PathBuf::from("change.diff"))),
        (
            Some(PathBuf::from("session.jsonl")),
            Some(PathBuf::from("change.diff")),
        ),
    ] {
        let input = SubstrateInput::new(session.clone(), diff.clone()).expect("input is valid");

        assert_eq!(session, input.session);
        assert_eq!(diff, input.diff);
    }
}

#[test]
fn substrate_input_rejects_empty_session_path() {
    let error = SubstrateInput::new(Some(PathBuf::from("")), None)
        .expect_err("empty session path rejected");

    assert!(matches!(error, BsmellError::SessionInputMalformed(_)));
}

#[test]
fn substrate_input_rejects_empty_diff_path() {
    let error =
        SubstrateInput::new(None, Some(PathBuf::from(""))).expect_err("empty diff path rejected");

    assert!(matches!(error, BsmellError::DiffSliceInvalid(_)));
}
