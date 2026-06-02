use crate::BsmellError;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(
    Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum InvocationSurface {
    #[default]
    L2aCli,
    L2bPayload,
    L2bStrapi,
    L2bSanity,
    L2bDirectus,
}

impl InvocationSurface {
    pub const ALL: [Self; 5] = [
        Self::L2aCli,
        Self::L2bPayload,
        Self::L2bStrapi,
        Self::L2bSanity,
        Self::L2bDirectus,
    ];

    pub const fn stable_name(self) -> &'static str {
        match self {
            Self::L2aCli => "l2a-cli",
            Self::L2bPayload => "l2b-payload",
            Self::L2bStrapi => "l2b-strapi",
            Self::L2bSanity => "l2b-sanity",
            Self::L2bDirectus => "l2b-directus",
        }
    }
}

impl fmt::Display for InvocationSurface {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.stable_name())
    }
}

impl FromStr for InvocationSurface {
    type Err = BsmellError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "l2a-cli" => Ok(Self::L2aCli),
            "l2b-payload" => Ok(Self::L2bPayload),
            "l2b-strapi" => Ok(Self::L2bStrapi),
            "l2b-sanity" => Ok(Self::L2bSanity),
            "l2b-directus" => Ok(Self::L2bDirectus),
            _ => Err(BsmellError::InvocationSurfaceUnknown(value.to_owned())),
        }
    }
}
