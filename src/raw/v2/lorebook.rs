use crate::raw::ext::Extensions;
use crate::raw::{shm, Content};
use serdev::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lorebook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_depth: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_budget: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive_scanning: Option<bool>,
    pub extensions: Extensions,
    pub entries: Vec<shm::LorebookEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EntryPosition {
    BeforeChar,
    AfterChar,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Id {
    Number(u64),
    String(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LorebookEntry {
    pub keys: Vec<String>,
    pub content: Content,
    pub extensions: Extensions,
    pub enabled: bool,
    pub insertion_order: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<EntryPosition>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub secondary_keys: Vec<String>,

    /// ***Non-Standard Item** (found in RisuAI)*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// ***Non-Standard Item** (found in RisuAI)*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
}
