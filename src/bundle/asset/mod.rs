use crate::bundle::asset::string::TextBundle;

pub mod string;

#[derive(Debug, Clone)]
pub struct AssetBundle {
    text: TextBundle
}

impl AssetBundle {
    pub fn new(text: TextBundle) -> Self {
        Self { text }
    }
}
