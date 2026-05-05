use crate::raw::v3::asset::Asset;
use crate::raw::Error;
use isolang::Language;
use jiff::Timestamp;
use serdev::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(validate = "Self::validate")]
pub struct CharacterCardData {
    #[serde(default)]
    pub assets: Vec<Asset>,

    pub nickname: Option<String>,

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub creator_notes_multilingual: HashMap<Language, String>,

    #[serde(default)]
    pub source: Vec<String>,

    pub group_only_greetings: Vec<String>,

    #[serde(with = "flexible_timestamp")]
    pub creation_date: Option<Timestamp>,

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
