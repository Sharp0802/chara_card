use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::raw::option::StrictOption;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCardData {
    pub creator_notes: String,
    pub system_prompt: String,
    pub post_history_instructions: String,
    pub alternate_greetings: Vec<String>,

    pub character_book: Option<crate::raw::Lorebook>,

    #[serde(flatten)]
    pub version2_0: StrictOption<Version2_0>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Version2_0 {
    pub tags: Vec<String>,
    pub creator: String,
    pub character_version: String,

    #[serde(default)]
    pub extensions: Map<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LorebookEntry {
    pub keys: Vec<String>,
    pub content: String,

    #[serde(default)]
    pub extensions: Map<String, Value>,
    pub enabled: bool,
    pub insertion_order: i32,
    pub case_sensitive: Option<bool>,
    pub name: Option<String>,
    pub priority: Option<i32>,
    pub id: Option<crate::raw::EntryId>,
    pub comment: Option<String>,
    pub selective: Option<bool>,
    pub secondary_keys: Option<Vec<String>>,
    pub constant: Option<bool>,
    pub position: Option<String>, // e.g., "before_char", "after_char"
}
