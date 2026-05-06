use crate::raw::ext::Extensions;
use crate::raw::v2::lorebook::Lorebook;
use serdev::{Deserialize, Serialize};

/// Represents version-2-specific features of character card data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCardData {
    /// Represents an array of string tag.
    ///
    /// ### Expected Behaviour
    ///
    /// Applications **MAY** use this field for sorting or filtering,
    /// which **SHOULD** be treated as **case-insensitive**.
    ///
    /// According to the specification,
    /// there are no restrictions on the content of the strings
    /// (including empty string).
    ///
    /// ### Implementation Notes
    ///
    /// To ensure consistency across the system,
    /// tags **SHOULD** be stored in lower case.
    ///
    /// Applications **MAY** transform user input
    /// to lower case automatically.
    ///
    /// While the application **MUST** accept existing empty string tags
    /// to remain compliant with the specification,
    /// it **SHOULD** ignore them during active processing or display.
    ///
    /// To maintain data quality and a clean UI,
    /// Applications **SHOULD** deny a user's attempt to
    /// manually add an empty string to the tags list.
    pub tags: Vec<String>,

    /// Represents name of creator.
    pub creator: String,

    /// Represents version string of character card.
    pub character_version: String,

    /// Represents replacement of global system prompt.
    ///
    /// ### Expected Behaviour
    ///
    /// In default behaviour,
    /// The global system prompt **MUST** be replaced with this field,
    /// and `{{original}}` placeholder **MUST** be replaced with original global system prompt.
    ///
    /// Applications **MAY** add how to handle original system prompt and this field,
    /// But it **MUST NOT** be the default behaviour.
    pub system_prompt: String,

    /// Represents post-history instructions.
    ///
    /// ### Expected Behaviour
    ///
    /// In default behaviour,
    /// The global jailbreak prompt **MUST** be replaced with this field,
    /// and `{{original}}` placeholder **MUST** be replaced with original global jailbreak prompt.
    ///
    /// Applications **MAY** add how to handle original jailbreak prompt and this field,
    /// But it **MUST NOT** be the default behaviour.
    pub post_history_instructions: String,

    /// Represents an array of alternative greeting strings.
    pub alternate_greetings: Vec<String>,

    /// Represents application-specific extensions.
    pub extensions: Extensions,

    /// Represents creator notes.
    ///
    /// ### Expected Behaviour
    ///
    /// If `creator_notes_multilingual` is not empty,
    /// this field **SHOULD** be considered as for `en` language.
    pub creator_notes: String,

    /// Represents lorebook.
    ///
    /// See [`v2::Lorebook`](Lorebook).
    pub character_book: Option<Lorebook>,
}
