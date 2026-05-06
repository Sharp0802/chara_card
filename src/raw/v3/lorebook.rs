use serdev::{Deserialize, Serialize};

/// Represents version-3-specific features of lorebook entry.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LorebookEntry {
    /// If true, strings in `keys` are considered as regex string.
    /// Otherwise, strings in `keys` are considered as regular string.
    ///
    /// ## Expected Behaviour
    ///
    /// If this field is true and `keys` has invalid regex string,
    /// Application **MUST** consider the lorebook as not a match.
    pub use_regex: bool,
}
