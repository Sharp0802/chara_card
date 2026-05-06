# `chara_card` &emsp; [![Version]][crates.io] [![License]][crates.io]

[Version]: https://img.shields.io/crates/v/chara_card.svg
[License]: https://img.shields.io/crates/l/chara_card.svg
[crates.io]: https://crates.io/crates/chara_card

- [**한국어**](README.kr.md)
- [**English**](README.md)

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
- [x] Non-standard field compatibility
    - [x] RisuAI (*testing...*)

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

Add `chara_card` to your dependencies in `Cargo.toml`.

**Parse `card.json`**:

```rust
use chara_card::raw::CharacterCard;

fn parse_card_json(card_json: &str) {
    // Parse character card from JSON string
    let parsed: CharacterCard = serde_json::from_str(card_json).unwrap();

    println!("{:#?}", parsed);
}
```

**Extract from `.charx`**:

> [!WARNING]
> Currently, `charx` module is unstable and API may be changed in future.

```rust
use chara_card::charx::CharX;

fn parse_charx(bytes: &[u8]) {
    // Extract from archived bytes (.charx)
    let reader = Cursor::new(bytes);
    let _charx = CharX::from(reader).unwrap();
}
```

## License

Licensed under [Apache License, Version 2.0](LICENSE).
