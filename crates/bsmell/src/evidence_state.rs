use crate::BsmellError;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceState {
    Clean,
    SmellDetected,
    Malformed,
}

impl EvidenceState {
    pub const ALL: [Self; 3] = [Self::Clean, Self::SmellDetected, Self::Malformed];

    pub const fn stable_name(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::SmellDetected => "smell-detected",
            Self::Malformed => "malformed",
        }
    }
}

impl fmt::Display for EvidenceState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.stable_name())
    }
}

impl FromStr for EvidenceState {
    type Err = BsmellError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "clean" => Ok(Self::Clean),
            "smell-detected" => Ok(Self::SmellDetected),
            "malformed" => Ok(Self::Malformed),
            _ => Err(BsmellError::EvidenceStateUnknown(value.to_owned())),
        }
    }
}

impl From<EvidenceState> for bsuite_core::ExitCode {
    fn from(value: EvidenceState) -> Self {
        match value {
            EvidenceState::Clean => Self::Success,
            EvidenceState::SmellDetected => Self::Finding,
            EvidenceState::Malformed => Self::InternalError,
        }
    }
}
