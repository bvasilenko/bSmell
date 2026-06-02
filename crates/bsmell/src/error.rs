use thiserror::Error;

#[derive(Debug, Error)]
pub enum BsmellError {
    #[error("session input is malformed: {0}")]
    SessionInputMalformed(String),
    #[error("diff slice is invalid: {0}")]
    DiffSliceInvalid(String),
    #[error("unknown smell category: {0}")]
    TaxonomyUnknown(String),
    #[error("unknown evidence state: {0}")]
    EvidenceStateUnknown(String),
    #[error("unknown invocation surface: {0}")]
    InvocationSurfaceUnknown(String),
    #[error("argument usage is invalid: {0}")]
    Usage(String),
    #[error(transparent)]
    Core(#[from] bsuite_core::BsuiteCoreError),
}

impl BsmellError {
    pub const fn exit_code(&self) -> bsuite_core::ExitCode {
        match self {
            Self::Usage(_) => bsuite_core::ExitCode::Usage,
            Self::SessionInputMalformed(_)
            | Self::DiffSliceInvalid(_)
            | Self::TaxonomyUnknown(_)
            | Self::EvidenceStateUnknown(_)
            | Self::InvocationSurfaceUnknown(_)
            | Self::Core(_) => bsuite_core::ExitCode::InternalError,
        }
    }

    pub fn process_exit_code(&self) -> std::process::ExitCode {
        process_exit_code(self.exit_code())
    }
}

pub fn process_exit_code(code: bsuite_core::ExitCode) -> std::process::ExitCode {
    std::process::ExitCode::from(code.as_i32() as u8)
}
