mod version_specific;

use crate::raw::Error;
use crate::raw::{v1, v2, v3, Version};
use serdev::{Deserialize, Serialize};

/// Represents character card regardless of specification version.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CharacterCard {
    /// Represents character card, version 1.
    Flat(v1::CharacterCardData),

    /// Represents character card, version 2 or later.
    Nested(NestedCharacterCard),
}

impl CharacterCard {
    /// Converts form of flat character card to nested form.
    ///
    /// "chara_card_v1" and "1.0" will be used for
    /// `spec` field and `spec_version` field of
    /// character card data.
    ///
    /// It has no effect for nested character card.
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

    /// Migrates character card to specific version.
    ///
    /// Downgrade can cause loss of version-specific data.
    pub fn migrate_to(self, version: Version) -> Result<CharacterCard, Error> {
        let Some(spec) = version.name() else {
            return Err(Error::UnsupportedVersion(version));
        };

        let mut nested = self.to_nested();

        if version < Version::V2 {
            return Ok(Self::Flat(nested.data.v1));
        }

        nested.override_v2(version >= Version::V2);
        nested.override_v3(version >= Version::V3);

        nested.spec = spec.to_owned();
        nested.spec_version = version;

        Ok(Self::Nested(nested))
    }
}

/// Represents nested form of character card.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(validate = "Self::validate")]
pub struct NestedCharacterCard {
    /// Represents specification name of the character card.
    ///
    /// To be consistent with other versions,
    /// "chara_card_v1" is used for CCv1.
    pub spec: String,

    /// Represents specification version of the character card.
    ///
    /// To be consistent with other versions,
    /// "1.0" is used for CCv1.
    pub spec_version: Version,

    /// Represents inner data of the character card.
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

    fn override_v2(&mut self, fill: bool) {
        if !fill {
            self.data.v2 = None;
        } else if self.data.v2.is_none() {
            self.data.v2 = Some(Default::default());
        }
    }

    fn override_v3(&mut self, fill: bool) {
        if !fill {
            self.data.v3 = None;
        } else if self.data.v3.is_none() {
            self.data.v3 = Some(Default::default());
        }

        let Some(v2) = self.data.v2.as_mut() else {
            return;
        };

        let Some(book) = &mut v2.character_book else {
            return;
        };

        for entry in &mut book.entries {
            if !fill {
                entry.v3 = None;
            } else if entry.v3.is_none() {
                entry.v3 = Some(Default::default());
            }
        }
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
                v2: None,
                v3: None,
            },
        }
    }
}

/// Represents character card data regardless of specification version.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCardData {
    /// Represents version-1-specific data.
    #[serde(flatten)]
    pub v1: v1::CharacterCardData,

    /// Represents version-2-specific data.
    ///
    /// For version 2 or later, it's ensured that this field is not `None`.
    #[serde(with = "version_specific")]
    #[serde(flatten)]
    pub v2: Option<v2::CharacterCardData>,

    /// Represents version-3-specific data.
    ///
    /// For version 3 or later, it's ensured that this field is not `None`.
    #[serde(with = "version_specific")]
    #[serde(flatten)]
    pub v3: Option<v3::CharacterCardData>,
}

/// Represents lorebook regardless of specification version.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lorebook {
    /// Represents version-2-specific data.
    #[serde(flatten)]
    v2: v2::Lorebook,
}

/// Represents lorebook entry regardless of specification version.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LorebookEntry {
    /// Represents version-2-specific data.
    #[serde(flatten)]
    v2: v2::LorebookEntry,

    /// Represents version-3-specific data.
    #[serde(with = "version_specific")]
    #[serde(flatten)]
    v3: Option<v3::LorebookEntry>,
}
