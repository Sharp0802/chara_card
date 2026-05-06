use serdev::{Deserialize, Serialize};

/// **Not Enough Observed**
///
/// Represents miscellaneous extension.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DepthPrompt {
    pub depth: u64,
    pub prompt: String,
}
