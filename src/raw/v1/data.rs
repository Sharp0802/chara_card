use serdev::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents version-1-specific features of character card data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCardData {
    /// Represents name of the character card.
    pub name: String,

    /// Represents description of the character card.
    pub description: String,

    /// Represents personality of the character.
    pub personality: String,

    /// Represents main scenario of the character card.
    pub scenario: String,

    /// Represents main greeting of the character card.
    pub first_mes: String,

    /// Represents example conversations.
    #[serde(with = "examples")]
    pub mes_example: Vec<String>,
}

pub mod examples {
    use super::*;
    use std::borrow::Cow;

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
        let raw = Cow::<'static, str>::deserialize(deserializer)?;

        let vec: Vec<_> = raw
            .split("<START>\n")
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.to_owned())
            .collect();

        Ok(vec)
    }
}
