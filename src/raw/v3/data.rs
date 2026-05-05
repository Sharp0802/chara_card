use crate::raw::v3::asset::Asset;
use isolang::Language;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCardData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    assets: Vec<Asset>,

    nickname: Option<String>,

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    creator_notes_multilingual: HashMap<Language, String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    source: Vec<String>,

    group_only_greetings: Vec<String>,

    #[serde(with = "flexible_timestamp")]
    creation_date: Option<Timestamp>,

    #[serde(with = "flexible_timestamp")]
    modification_date: Option<Timestamp>,
}

// I don't know why this is required!!!
//
// Even though millisecond timestamp is NOT the standard,
// Some real-world data uses millisecond as timestamp.
// In addition, that sample uses seconds for modification date,
// milliseconds for creation date...
pub mod flexible_timestamp {
    use super::*;
    use serde::{Deserializer, Serializer};

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
                .map_err(serde::de::Error::custom)?;
                Ok(Some(ts))
            }
            None => Ok(None),
        }
    }
}
