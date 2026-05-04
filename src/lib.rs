pub mod bundle;
pub mod raw;

#[cfg(test)]
mod tests {
    use super::*;

    const CARD_JSON: &str = include_str!("../tests/sample/card.json");

    #[test]
    fn parse_card_json() {
        let cc: raw::CharacterCard = serde_json::from_str(CARD_JSON).unwrap();

        let bundle: bundle::Bundle = cc.try_into().unwrap();

        println!("{:?}", bundle);
    }
}
