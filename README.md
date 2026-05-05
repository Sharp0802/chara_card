# `chara_card`

`chara_card` provides thick (not yet...) abstraction layer on
character card formats and `.charx` in Rust.

Its primary goal is to help programs to run
the **same format everywhere with same behaviour**.

## Features

**(De)serialization/Validation**:

- [x] Full Standard Compliance
  - [Character Card v1/v2 spec.](https://github.com/malfoyslastname/character-card-spec-v2)
  - [Character Card v3 spec.](https://github.com/kwaroran/character-card-spec-v3)
- [x] Content parsing
  - [x] Decorator parser
  - [x] Curly Braced Syntaxes (CBS) parser
- [ ] Non-standard field compatibility
  - [ ] RisuAI (*testing...*)

**Evaluation**:

- [ ] Content evaluation
  - [ ] Decorator evaluator
  - [ ] CBS evaluator
- [ ] Non-standard extension compatibility
  - [ ] RisuAI

**Packaging**:

- [x] Extracting from `.charx`
- [ ] Packaging to `.charx`

## Usage

> [!WARNING]
> Work in Progress

Add `chara_card` to your dependencies in `Cargo.toml`.

```rust
use chara_card::raw::CharacterCard;

fn parse_card_json(card_json: &str) {
    // Parse character card from JSON string
    let parsed: CharacterCard = serde_json::from_str(card_json).unwrap();
    
    println!("{:#?}", parsed);
}
```

## License

Licensed under [Apache License, Version 2.0](LICENSE).
