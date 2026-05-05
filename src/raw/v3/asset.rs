use serde::{Deserialize, Serialize};
use uriparse::URI;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    r#type: String,
    uri: URI<'static>,
    name: String,
    ext: String,
}
