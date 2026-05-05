use serdev::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LorebookEntry {
    pub use_regex: bool,
}
