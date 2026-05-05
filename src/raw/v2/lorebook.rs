use crate::raw::{shm, Content};
use crate::raw::v2::extension::Extensions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lorebook {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scan_depth: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_budget: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    case_sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    constant: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selective: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<EntryPosition>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    secondary_keys: Vec<String>,

    /// ***Non-Standard Item** (found in RisuAI)*
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<String>,

    /// ***Non-Standard Item** (found in RisuAI)*
    #[serde(skip_serializing_if = "Option::is_none")]
    folder: Option<String>,
}
