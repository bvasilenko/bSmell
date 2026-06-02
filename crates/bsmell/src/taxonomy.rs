use crate::BsmellError;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SmellCategory {
    TryExceptSwallow,
    HardcodedValue,
    MockInsteadOfFix,
    SilentSuccess,
    ScopeCreep,
    ScopeShrink,
    PrideDefense,
    TimeSink,
    RedHerring,
    SymptomMute,
    UnauditedEdit,
    AcceptableDegradation,
    SchemaViolationSilentCoerce,
    RegulatedTermOmission,
    SynonymHallucination,
}

impl SmellCategory {
    pub const ALL: [Self; 15] = [
        Self::TryExceptSwallow,
        Self::HardcodedValue,
        Self::MockInsteadOfFix,
        Self::SilentSuccess,
        Self::ScopeCreep,
        Self::ScopeShrink,
        Self::PrideDefense,
        Self::TimeSink,
        Self::RedHerring,
        Self::SymptomMute,
        Self::UnauditedEdit,
        Self::AcceptableDegradation,
        Self::SchemaViolationSilentCoerce,
        Self::RegulatedTermOmission,
        Self::SynonymHallucination,
    ];

    pub const fn stable_name(self) -> &'static str {
        match self {
            Self::TryExceptSwallow => "try-except-swallow",
            Self::HardcodedValue => "hardcoded-value",
            Self::MockInsteadOfFix => "mock-instead-of-fix",
            Self::SilentSuccess => "silent-success",
            Self::ScopeCreep => "scope-creep",
            Self::ScopeShrink => "scope-shrink",
            Self::PrideDefense => "pride-defense",
            Self::TimeSink => "time-sink",
            Self::RedHerring => "red-herring",
            Self::SymptomMute => "symptom-mute",
            Self::UnauditedEdit => "unaudited-edit",
            Self::AcceptableDegradation => "acceptable-degradation",
            Self::SchemaViolationSilentCoerce => "schema-violation-silent-coerce",
            Self::RegulatedTermOmission => "regulated-term-omission",
            Self::SynonymHallucination => "synonym-hallucination",
        }
    }
}

impl fmt::Display for SmellCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.stable_name())
    }
}

impl FromStr for SmellCategory {
    type Err = BsmellError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "try-except-swallow" => Ok(Self::TryExceptSwallow),
            "hardcoded-value" => Ok(Self::HardcodedValue),
            "mock-instead-of-fix" => Ok(Self::MockInsteadOfFix),
            "silent-success" => Ok(Self::SilentSuccess),
            "scope-creep" => Ok(Self::ScopeCreep),
            "scope-shrink" => Ok(Self::ScopeShrink),
            "pride-defense" => Ok(Self::PrideDefense),
            "time-sink" => Ok(Self::TimeSink),
            "red-herring" => Ok(Self::RedHerring),
            "symptom-mute" => Ok(Self::SymptomMute),
            "unaudited-edit" => Ok(Self::UnauditedEdit),
            "acceptable-degradation" => Ok(Self::AcceptableDegradation),
            "schema-violation-silent-coerce" => Ok(Self::SchemaViolationSilentCoerce),
            "regulated-term-omission" => Ok(Self::RegulatedTermOmission),
            "synonym-hallucination" => Ok(Self::SynonymHallucination),
            _ => Err(BsmellError::TaxonomyUnknown(value.to_owned())),
        }
    }
}
