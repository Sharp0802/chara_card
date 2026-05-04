use std::collections::HashMap;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCardData {
    pub assets: Option<Vec<Asset>>,
    pub nickname: Option<String>,
    pub creator_notes_multilingual: Option<HashMap<String, String>>,
    pub source: Option<Vec<String>>,

    pub group_only_greetings: Vec<String>,

    #[serde(with = "jiff::fmt::serde::timestamp::second::optional")]
    pub creation_date: Option<Timestamp>,

    #[serde(with = "jiff::fmt::serde::timestamp::second::optional")]
    pub modification_date: Option<Timestamp>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LorebookEntry {
    pub use_regex: bool,
    pub constant: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    pub r#type: String,
    pub uri: String,
    pub name: String,
    pub ext: String,
}
