use crate::BsmellError;
use std::path::PathBuf;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SubstrateInput {
    pub session: Option<PathBuf>,
    pub diff: Option<PathBuf>,
}

impl SubstrateInput {
    pub fn new(session: Option<PathBuf>, diff: Option<PathBuf>) -> Result<Self, BsmellError> {
        if matches!(session.as_ref().and_then(|path| path.to_str()), Some("")) {
            return Err(BsmellError::SessionInputMalformed(
                "session path must not be empty".to_owned(),
            ));
        }

        if matches!(diff.as_ref().and_then(|path| path.to_str()), Some("")) {
            return Err(BsmellError::DiffSliceInvalid(
                "diff path must not be empty".to_owned(),
            ));
        }

        Ok(Self { session, diff })
    }
}
