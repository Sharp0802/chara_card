use assert_json_diff::assert_json_eq;
use chara_card::charx::CharX;
use chara_card::raw::CharacterCard;
use jiff::Timestamp;
use rstest::rstest;
use serde_json::Value;
use std::fs::{read_to_string, File};
use std::path::PathBuf;
use url::Url;

#[rstest]
fn parse_charx(#[files("tests/samples/charx/*.charx")] path: PathBuf) {
    let file = File::open(path).unwrap();
    let _charx = CharX::from(file).unwrap();
}

#[rstest]
fn parse_card_json(#[files("tests/samples/json/*/card.json")] path: PathBuf) {
    let json = read_to_string(path).unwrap();

    let mut value: Value = serde_json::from_str(&json).unwrap();

    // NOTE: RisuAI uses millisecond for creation timestamp
    if let Some(creation_date) = value
        .get_mut("data")
        .and_then(|v| v.get_mut("creation_date"))
    {
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

    let dto: CharacterCard = serde_json::from_str(&json).unwrap();
    let ser = serde_json::to_value(dto).unwrap();

    assert_json_eq!(value, ser);
}
