mod model;

#[cfg(test)]
mod tests {
    const CARD_JSON: &str = include_str!("../tests/sample/card.json");

    #[test]
    fn parse_card_json() {
        let cc: CharacterCard = serde_json::from_str(CARD_JSON).unwrap();
        println!("{:?}", cc);
    }
}
