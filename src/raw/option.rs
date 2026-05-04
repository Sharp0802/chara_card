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
                return Ok(Self(None));
            }
        }

        match T::deserialize(v) {
            Ok(val) => Ok(Self(Some(val))),
            Err(_) => Ok(Self(None)),
        }
    }
}

impl<T> StrictOption<T> {
    // NOTE: Not using `Into` is intended.
    //       Into<Option<T>> is not inferred from `let Option(_) = strict_option.into()`
    pub fn into(self) -> Option<T> {
        self.0
    }
}

impl<T> AsRef<Option<T>> for StrictOption<T> {
    fn as_ref(&self) -> &Option<T> {
        &self.0
    }
}

impl<T> From<Option<T>> for StrictOption<T> {
    fn from(v: Option<T>) -> Self {
        Self(v)
    }
}
