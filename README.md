# `chara_card`

`chara_card` provides thick abstraction layer on
character card formats and `.charx` in Rust.

- [Character Card v1/v2 spec.](https://github.com/malfoyslastname/character-card-spec-v2)
- [Character Card v3 spec.](https://github.com/kwaroran/character-card-spec-v3)

## Usage

> [!WARNING]
> Work in Progress

```rust
use chara_card::{raw, bundle};

fn parse_card_json() {
    let cc: raw::CharacterCard = serde_json::from_str(CARD_JSON).unwrap();
    
    let bundle: bundle::Bundle = cc.try_into().unwrap();
    
    println!("{:?}", bundle);
}
```

## License

Licensed under [Apache License, Version 2.0](LICENSE).
