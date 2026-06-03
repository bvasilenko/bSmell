use bsmell::BsmellError;
use bsuite_core::{BsuiteCoreError, ExitCode};

fn assert_error_exit_code(error: BsmellError, expected: ExitCode) {
    assert_eq!(expected, error.exit_code());
    assert_eq!(
        std::process::ExitCode::from(expected.as_i32() as u8),
        error.process_exit_code()
    );
}

#[test]
fn usage_errors_map_to_usage_exit_code() {
    assert_error_exit_code(
        BsmellError::Usage("bad arguments".to_owned()),
        ExitCode::Usage,
    );
}

#[test]
fn domain_errors_map_to_internal_error_exit_code() {
    for error in [
        BsmellError::SessionInputMalformed("bad session".to_owned()),
        BsmellError::DiffSliceInvalid("bad diff".to_owned()),
        BsmellError::TaxonomyUnknown("bad taxonomy".to_owned()),
        BsmellError::EvidenceStateUnknown("bad state".to_owned()),
        BsmellError::InvocationSurfaceUnknown("bad surface".to_owned()),
        BsmellError::Core(BsuiteCoreError::PromptResolution("bad prompt".to_owned())),
    ] {
        assert_error_exit_code(error, ExitCode::InternalError);
    }
}
