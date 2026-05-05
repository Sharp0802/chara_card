use serdev::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LorebookEntry {
    use_regex: bool,
}
