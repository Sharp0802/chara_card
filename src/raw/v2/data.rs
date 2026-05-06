use serdev::{Deserialize, Serialize};
use crate::raw::ext::Extensions;
use crate::raw::v2::lorebook::Lorebook;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCardData {
    pub tags: Vec<String>,
    pub creator: String,
    pub character_version: String,
    pub system_prompt: String,
    pub post_history_instructions: String,
    pub alternate_greetings: Vec<String>,
    pub extensions: Extensions,
    pub creator_notes: String,
    pub character_book: Option<Lorebook>,
}
