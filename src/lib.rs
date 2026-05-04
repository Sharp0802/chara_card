mod v1;
mod v2;
mod v3;
mod option;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::option::StrictOption;

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
    pub data: Option<CharacterCardData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCardData {
    #[serde(flatten)]
    pub v1: v1::CharacterCardData,

    #[serde(flatten)]
    pub v2: v2::CharacterCardData,

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


#[cfg(test)]
mod tests {
    use super::*;

    const CARD_JSON: &str = include_str!("../tests/sample/card.json");

    #[test]
    fn parse_card_json() {
        let cc: CharacterCard = serde_json::from_str(CARD_JSON).unwrap();
        println!("{:?}", cc);
    }
}
