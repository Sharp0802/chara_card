mod option;

pub mod v1;
pub mod v2;
pub mod v3;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub use crate::raw::option::StrictOption;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CharacterCard {
    Nested(NestedCharacterCard),
    Flat(v1::CharacterCardData),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NestedCharacterCard {
    pub spec: String,
    pub spec_version: String,
    pub data: CharacterCardData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCardData {
    #[serde(flatten)]
    pub v1: v1::CharacterCardData,

    #[serde(flatten)]
    pub v2: StrictOption<v2::CharacterCardData>,

    #[serde(flatten)]
    pub v3: StrictOption<v3::CharacterCardData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lorebook {
    pub name: Option<String>,
    pub description: Option<String>,
    pub scan_depth: Option<u32>,
    pub token_budget: Option<u32>,
    pub recursive_scanning: Option<bool>,

    #[serde(default)]
    pub extensions: Map<String, Value>,
    pub entries: Vec<LorebookEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LorebookEntry {
    #[serde(flatten)]
    pub v2: v2::LorebookEntry,

    #[serde(flatten)]
    pub v3: StrictOption<v3::LorebookEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EntryId {
    Number(i64),
    String(String),
}
