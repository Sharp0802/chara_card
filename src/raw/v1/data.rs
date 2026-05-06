use crate::raw::v1::examples::Examples;
use serdev::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCardData {
    pub name: String,
    pub description: String,
    pub personality: String,
    pub scenario: String,
    pub first_mes: String,
    pub mes_example: String,
    pub mes_example: Examples,
    #[serde(with = "examples")]
    pub mes_example: Vec<String>,
}

pub mod examples {
    use super::*;

    pub fn serialize<S>(value: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buffer = String::new();
        for item in value {
            buffer.push_str("<START>\n");
            buffer.push_str(item);
        }
        serializer.serialize_str(&buffer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = <&str>::deserialize(deserializer)?;

        let vec: Vec<_> = raw
            .split("<START>\n")
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.to_owned())
            .collect();

        Ok(vec)
    }
}
