use chara_card::raw::CharacterCard;

const CARDS: [&str; 2] = [
    include_str!("samples/sample1/card.json"),
    include_str!("samples/sample2/card.json"),
];

fn main() {
    for card in CARDS {
        let _parsed: CharacterCard = serde_json::from_str(card).unwrap();
    }
}
