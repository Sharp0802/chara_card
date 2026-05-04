use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct StrictOption<T>(pub Option<T>);

impl<T: Serialize> Serialize for StrictOption<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for StrictOption<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let v = Value::deserialize(deserializer)?;

        if let Value::Object(ref map) = v {
            if map.is_empty() {
                return Ok(StrictOption(None));
            }
        }

        T::deserialize(v)
            .map(|val| StrictOption(Some(val)))
            .map_err(serde::de::Error::custom)
    }
}
