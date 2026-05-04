use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCardData {
    // Using empty-string instead of option is intended and required by specification.
    // See docs/spec_v1.md.

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub personality: String,

    #[serde(default)]
    pub scenario: String,

    #[serde(default)]
    pub first_mes: String,

    #[serde(default)]
    pub mes_example: String,
}
