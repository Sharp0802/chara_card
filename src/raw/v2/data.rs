use serde::{Deserialize, Serialize};

use crate::raw::v2::extension::Extensions;
use crate::raw::v2::lorebook::Lorebook;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCardData {
    tags: Vec<String>,
    creator: String,
    character_version: String,
    system_prompt: String,
    post_history_instructions: String,
    alternate_greetings: Vec<String>,
    extensions: Extensions,
    creator_notes: String,
    character_book: Option<Lorebook>,
}
