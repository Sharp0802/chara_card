use serdev::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCardData {
    name: String,
    description: String,
    personality: String,
    scenario: String,
    first_mes: String,
    mes_example: String,
}
