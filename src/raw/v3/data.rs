use crate::raw::v3::asset::Asset;
use crate::raw::Error;
use isolang::Language;
use jiff::Timestamp;
use serdev::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents version-3-specific features of character card data.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(validate = "Self::validate")]
pub struct CharacterCardData {
    /// Represents an array of asset.
    ///
    /// ### Compatibility Notes
    ///
    /// While the specification suggests that `name` field within an `Asset`
    /// **MAY** be used as an identifier,
    /// some implementations (tested with *RisuAI*, at 2026-05-05)
    /// permit duplicate names within this array.
    ///
    /// Consequently, the `name` field **SHOULD NOT** be treated as guaranteed unique key.
    ///
    #[serde(default)]
    pub assets: Vec<Asset>,

    /// Represents replacement of character name.
    ///
    /// ### Expected Behaviour
    ///
    /// If set, character name placeholders
    /// (e.g., `{{char}}`, `<char>` or `<bot>`)
    /// **SHOULD** be replaced with this value.
    ///
    /// ### Implementation Notes
    ///
    /// Even if this field is set,
    /// `name` **SHOULD** be used as identifier of character card.
    pub nickname: Option<String>,

    /// Represents multilingual creator notes.
    ///
    /// ### Expected Behaviour
    ///
    /// If this field is not empty,
    /// `creator_notes` **SHOULD** be considered as for `en` language.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub creator_notes_multilingual: HashMap<Language, String>,

    /// Represents an array of the ID or URL that points to the citations.
    ///
    /// ### Implementation Notes
    ///
    /// The specification **did not define** the exact format or semantics of the IDs.
    ///
    /// Therefore,
    /// while an application **MAY** assign application-specific semantics
    /// to these IDs within its own ecosystem,
    ///
    /// It **SHOULD NOT** allow users to add new non-URL sources.
    /// Furthermore, IDs **SHOULD NOT** be relied upon for cross-application compatibility.
    #[serde(default)]
    pub source: Vec<String>,

    /// Represents an array of additional greetings string only used for group chats.
    ///
    /// ### Implementation Notes
    ///
    /// The specification describes this field as **additional** greetings;
    /// However, it's unclear whether this is intended as a supplement or
    /// a legacy consideration for CCv1 (`first_mes`) compatibility.
    ///
    /// To ensure a consistent user experience,
    /// If this field is not empty, applications **SHOULD** use it
    /// as the exclusive source for group chat greetings,
    /// superseding the primary greetings.
    pub group_only_greetings: Vec<String>,

    /// Represents the creation timestamp.
    ///
    /// ### Compatibility Notes
    ///
    /// Although the specification defines UNIX timestamps with second-level precision,
    /// some implementations (tested with *RisuAI*, at 2026-05-05)
    /// may use millisecond-level precision for creation date.
    ///
    /// This crate **automatically normalizes** these values to the standard
    /// second-level precision during deserialization.
    /// When serializing, the value will always be written in standard seconds.
    #[serde(with = "flexible_timestamp")]
    pub creation_date: Option<Timestamp>,

    /// Represents the modification timestamp.
    #[serde(with = "flexible_timestamp")]
    pub modification_date: Option<Timestamp>,
}

impl CharacterCardData {
    fn validate(&self) -> Result<(), Error> {
        // let mut set = HashSet::with_capacity(self.assets.len());
        // for name in self.assets.iter().map(|asset| asset.name.as_str()) {
        //     if !set.insert(name) {
        //         return Err(Error::AssetNameConflict(name.to_owned()));
        //     }
        // }

        // Non-standard again!!!
        // Real-world data may contain duplicated asset names.
        // (checked with artefacts from RisuAI)
        // We bypass the strict check.

        Ok(())
    }
}

// I don't know why this is required!!!
//
// Even though millisecond timestamp is NOT the standard,
// Some real-world data uses millisecond as timestamp.
// In addition, that sample uses seconds for modification date,
// milliseconds for creation date...
pub mod flexible_timestamp {
    use super::*;
    use serdev::de::Error;
    use serdev::{Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<Timestamp>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(ts) => serializer.serialize_i64(ts.as_second()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Timestamp>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<i64> = Option::deserialize(deserializer)?;
        match opt {
            Some(v) => {
                // If it's larger than 1e11, it's likely milliseconds
                let ts = if v > 100_000_000_000 {
                    Timestamp::from_millisecond(v)
                } else {
                    Timestamp::from_second(v)
                }
                .map_err(D::Error::custom)?;
                Ok(Some(ts))
            }
            None => Ok(None),
        }
    }
}
