mod version_specific;

use crate::raw::{v1, v2, v3, Version};
use serdev::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CharacterCard {
    Flat(v1::CharacterCardData),
    Nested(NestedCharacterCard),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NestedCharacterCard {
    pub spec: String,
    pub spec_version: Version,
    pub data: CharacterCardData,
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
