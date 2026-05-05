use serdev::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    r#type: String,
    uri: Url,
    name: String,
    ext: String,
}
