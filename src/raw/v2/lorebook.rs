use crate::raw::{shm, Content};
use crate::raw::v2::extension::Extensions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lorebook {
    name: Option<String>,
    description: Option<String>,
    scan_depth: Option<u64>,
    token_budget: Option<u64>,
    recursive_scanning: Option<bool>,
    extensions: Extensions,
    entries: Vec<shm::LorebookEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EntryPosition {
    BeforeChar,
    AfterChar,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Id {
    Number(u64),
    String(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LorebookEntry {
    keys: Vec<String>,
    content: Content,
    extensions: Extensions,
    enabled: bool,
    insertion_order: u64,
    case_sensitive: Option<bool>,
    constant: Option<bool>,
    name: Option<String>,
    id: Option<Id>,
    comment: Option<String>,
    position: Option<EntryPosition>,
    priority: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    selective: Option<bool>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    secondary_keys: Vec<String>,
}
