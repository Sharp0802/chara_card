use serdev::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    pub r#type: String,
    pub uri: Url,
    pub name: String,
    pub ext: String,
}
