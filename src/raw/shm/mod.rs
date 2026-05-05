mod version_specific;

use crate::raw::Error;
use crate::raw::{v1, v2, v3, Version};
use serdev::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CharacterCard {
    Flat(v1::CharacterCardData),
    Nested(NestedCharacterCard),
}

impl CharacterCard {
    pub fn to_nested(self) -> NestedCharacterCard {
        let flat = match self {
            CharacterCard::Flat(flat) => flat,
            CharacterCard::Nested(nested) => return nested,
        };

        NestedCharacterCard {
            spec: Version::V1.name().unwrap().into(),
            spec_version: Version::V1,
            data: CharacterCardData {
                v1: flat,
                v2: None,
                v3: None,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(validate = "Self::validate")]
pub struct NestedCharacterCard {
    pub spec: String,
    pub spec_version: Version,
    pub data: CharacterCardData,
}

impl NestedCharacterCard {
    fn validate(&self) -> Result<(), Error> {
        let Some(spec) = self.spec_version.name() else {
            return Err(Error::UnsupportedVersion(self.spec_version));
        };

        if self.spec != spec {
            return Err(Error::UnsupportedSpec(self.spec.clone()));
        }

        if self.spec_version < Version::V2 {
            return Ok(());
        }

        let Some(v2) = &self.data.v2 else {
            return Err(Error::MissingFeature(Version::V2, "Data".into()));
        };

        if self.spec_version < Version::V3 {
            return Ok(());
        }

        if self.data.v3.is_none() {
            return Err(Error::MissingFeature(Version::V3, "Data".into()));
        }

        if let Some(book) = &v2.character_book {
            for entry in &book.entries {
                if entry.v3.is_some() {
                    continue;
                }

                return Err(Error::MissingFeature(Version::V3, "LorebookEntry".into()));
            }
        }

        Ok(())
    }
}

impl From<CharacterCard> for NestedCharacterCard {
    fn from(value: CharacterCard) -> Self {
        let v1 = match value {
            CharacterCard::Flat(v1) => v1,
            CharacterCard::Nested(nested) => return nested,
        };

        Self {
            spec: Version::V1.name().unwrap().into(),
            spec_version: Version::V1,
            data: CharacterCardData {
                v1,
                v2: None.into(),
                v3: None.into(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCardData {
    #[serde(flatten)]
    pub v1: v1::CharacterCardData,

    #[serde(with = "version_specific")]
    #[serde(flatten)]
    pub v2: Option<v2::CharacterCardData>,

    #[serde(with = "version_specific")]
    #[serde(flatten)]
    pub v3: Option<v3::CharacterCardData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lorebook {
    #[serde(flatten)]
    v2: v2::Lorebook,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LorebookEntry {
    #[serde(flatten)]
    v2: v2::LorebookEntry,

    #[serde(with = "version_specific")]
    #[serde(flatten)]
    v3: Option<v3::LorebookEntry>,
}
