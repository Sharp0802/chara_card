use serdev::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S, T: Serialize>(option: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    option.serialize(serializer)
}

pub fn deserialize<'de, D, T: Deserialize<'de>>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
{
    match T::deserialize(deserializer) {
        Ok(val) => Ok(Some(val)),
        Err(_) => Ok(None),
    }
}
