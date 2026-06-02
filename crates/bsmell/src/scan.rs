use crate::{BsmellError, ScanArgs, substrate_input::SubstrateInput};
use std::process::ExitCode;

const ROUTING_KEY: &str = "bsmell";
const PLACEHOLDER_STATUS: &str = "placeholder";
const DETECTION_STATE: &str = "not-run";
const DEFERRED_REASON: &str = "detection behavior is deferred";

pub fn run(args: ScanArgs) -> Result<ExitCode, BsmellError> {
    let input = SubstrateInput::new(args.session, args.diff)?;
    validate_reason(args.reason.as_deref())?;

    if !args.quiet {
        // Deferred to a later bsmell package cycle so scan output cannot be mistaken for detection.
        DeferredScanReport::new(input).write(args.json);
    }

    Ok(ExitCode::SUCCESS)
}

fn validate_reason(reason: Option<&str>) -> Result<(), BsmellError> {
    if matches!(reason, Some(reason) if reason.trim().is_empty()) {
        return Err(BsmellError::Usage("reason must not be empty".to_owned()));
    }

    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct DeferredScanReport {
    has_session: bool,
    has_diff: bool,
}

impl DeferredScanReport {
    fn new(input: SubstrateInput) -> Self {
        Self {
            has_session: input.session.is_some(),
            has_diff: input.diff.is_some(),
        }
    }

    fn write(&self, json: bool) {
        if json {
            println!(
                "{{\"status\":\"{PLACEHOLDER_STATUS}\",\"routing_key\":\"{ROUTING_KEY}\",\"detection\":\"{DETECTION_STATE}\",\"reason\":\"{DEFERRED_REASON}\",\"inputs\":{{\"session\":{},\"diff\":{}}}}}",
                self.has_session, self.has_diff
            );
        } else {
            println!("bsmell scan placeholder: {DEFERRED_REASON}; detection={DETECTION_STATE}.");
        }
    }
}
