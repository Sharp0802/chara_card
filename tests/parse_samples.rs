use assert_json_diff::assert_json_eq;
use jiff::Timestamp;
use serde_json::Value;
use url::Url;
use chara_card::raw::CharacterCard;

const CARDS: [&str; 2] = [
    include_str!("samples/sample1/card.json"),
    include_str!("samples/sample2/card.json"),
];

#[test]
fn parse_samples() {
    for card in CARDS {
        let mut value: Value = serde_json::from_str(card).unwrap();

        // NOTE: RisuAI uses millisecond for creation timestamp
        if let Some(creation_date) = value.get_mut("data").and_then(|v| v.get_mut("creation_date")) {
            let ts = creation_date.as_i64().unwrap();
            let ts = Timestamp::from_millisecond(ts).unwrap();
            *creation_date = Value::Number(ts.as_second().into())
        }
        
        // NOTE: It seems RisuAI doesn't encode URIs
        if let Some(assets) = value.get_mut("data").and_then(|v| v.get_mut("assets")) {
            for asset in assets.as_array_mut().unwrap() {
                let Some(uri) = asset.get_mut("uri") else {
                    continue;
                };

                let url: Url = uri.as_str().unwrap().parse().unwrap();

                *uri = Value::String(url.to_string());
            }
        }

        let dto: CharacterCard = serde_json::from_str(card).unwrap();
        let ser = serde_json::to_value(dto).unwrap();

        assert_json_eq!(value, ser);
    }
}
