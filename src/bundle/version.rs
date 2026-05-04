use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

use crate::bundle::Error;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

impl Version {
    pub const V1: Self = Self { major: 1, minor: 0 };
    pub const V2: Self = Self { major: 2, minor: 0 };
    pub const V3: Self = Self { major: 3, minor: 0 };

    pub fn name(&self) -> Option<&'static str> {
        match self.major {
            1 => Some("chara_card_v1"),
            2 => Some("chara_card_v2"),
            3 => Some("chara_card_v3"),
            _ => None,
        }
    }
}

impl From<(u8, u8)> for Version {
    fn from((major, minor): (u8, u8)) -> Self {
        Self { major, minor }
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let at = value.find('.').ok_or(Error::MalformedVersion)?;

        let (major, minor) = value.split_at(at);
        let major = major.parse().map_err(|_| Error::MalformedVersion)?;
        let minor = minor[1..].parse().map_err(|_| Error::MalformedVersion)?;

        Ok(Self { major, minor })
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}
