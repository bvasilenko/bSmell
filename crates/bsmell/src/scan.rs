use crate::{
    BsmellError, EvidenceState, InvocationSurface, ScanArgs, SmellCategory,
    error::process_exit_code, substrate_input::SubstrateInput,
};
use std::{fmt, path::Path};

const BINARY_NAME: &str = "bsmell";
const PLACEHOLDER_CATEGORY: SmellCategory = SmellCategory::RedHerring;
const PLACEHOLDER_EVIDENCE_STATE: EvidenceState = EvidenceState::SmellDetected;
const PLACEHOLDER_INVOCATION_SURFACE: InvocationSurface = InvocationSurface::L2aCli;

pub fn run(args: ScanArgs) -> Result<std::process::ExitCode, BsmellError> {
    let input = SubstrateInput::new(args.session, args.diff)?;
    validate_reason(args.reason.as_deref())?;

    println!("{}", PlaceholderDirective::new(input));

    Ok(process_exit_code(PLACEHOLDER_EVIDENCE_STATE.into()))
}

fn validate_reason(reason: Option<&str>) -> Result<(), BsmellError> {
    if matches!(reason, Some(reason) if reason.trim().is_empty()) {
        return Err(BsmellError::Usage("reason must not be empty".to_owned()));
    }

    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct PlaceholderDirective {
    input: SubstrateInput,
    category: SmellCategory,
    evidence_state: EvidenceState,
    invocation_surface: InvocationSurface,
}

impl PlaceholderDirective {
    fn new(input: SubstrateInput) -> Self {
        Self {
            input,
            category: PLACEHOLDER_CATEGORY,
            evidence_state: PLACEHOLDER_EVIDENCE_STATE,
            invocation_surface: PLACEHOLDER_INVOCATION_SURFACE,
        }
    }
}

impl fmt::Display for PlaceholderDirective {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            formatter,
            "[{BINARY_NAME} placeholder directive - pre-corpus output]"
        )?;
        writeln!(
            formatter,
            "Parsed input: session={}, diff={}. Routing key: SmellCategory::{} placeholder route. Invocation surface: {}. Evidence-state: {}.",
            format_optional_path(self.input.session.as_deref()),
            format_optional_path(self.input.diff.as_deref()),
            format_variant_name(self.category),
            self.invocation_surface.public_label(),
            format_variant_name(self.evidence_state),
        )?;
        writeln!(
            formatter,
            "ACTION: This invocation reached {BINARY_NAME} at the pre-corpus phase. A real evolved directive would name the specific deflection pattern {} matched in the session or diff and steer the calling LLM toward acknowledging and re-anchoring on the underlying task rather than the deflection.",
            self.category,
        )?;
        write!(
            formatter,
            "Re-invoke after the corpus-backed release lands. Do not treat this placeholder as ground truth. Exit code carries the verdict-class signal."
        )
    }
}

fn format_optional_path(path: Option<&Path>) -> String {
    path.map(|path| path.display().to_string())
        .unwrap_or_else(|| "<none>".to_owned())
}

fn format_variant_name<T>(value: T) -> String
where
    T: fmt::Debug,
{
    format!("{value:?}")
}
