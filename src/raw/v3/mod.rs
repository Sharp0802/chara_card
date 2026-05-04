use std::collections::HashMap;
use isolang::Language;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCardData {
    pub assets: Option<Vec<Asset>>,
    pub nickname: Option<String>,
    pub creator_notes_multilingual: Option<HashMap<Language, String>>,
    pub source: Option<Vec<String>>,

    pub group_only_greetings: Vec<String>,

    #[serde(with = "flexible_timestamp")]
    pub creation_date: Option<Timestamp>,

    #[serde(with = "flexible_timestamp")]
    pub modification_date: Option<Timestamp>,
}

pub mod flexible_timestamp {
    use serde::{Deserializer, Serializer};
    use super::*;

    pub fn serialize<S>(date: &Option<Timestamp>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(ts) => serializer.serialize_i64(ts.as_millisecond()),
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
                // I don't know why this is required!!!
                //
                // Even though millisecond timestamp is NOT the standard,
                // Some real-world data uses millisecond as timestamp.
                // In addition, that sample uses seconds for modification date,
                // milliseconds for creation date...
                //
                // If it's larger than 1e11, it's likely milliseconds
                let ts = if v > 100_000_000_000 {
                    Timestamp::from_millisecond(v)
                } else {
                    Timestamp::from_second(v)
                }.map_err(serde::de::Error::custom)?;
                Ok(Some(ts))
            }
            None => Ok(None),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LorebookEntry {
    pub use_regex: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    pub r#type: String,
    pub uri: String,
    pub name: String,
    pub ext: String,
}
