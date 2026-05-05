use serdev::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct Extensions(BTreeMap<String, Extension>);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Extension {
    Unknown(Value),
}
