# `chara_card` &emsp; [![Version]][crates.io] [![License]][crates.io]

[Version]: https://img.shields.io/crates/v/chara_card.svg
[License]: https://img.shields.io/crates/l/chara_card.svg
[crates.io]: https://crates.io/crates/chara_card`

- [**한국어**](README.kr.md)
- [**English**](README.md)

`chara_card`는 캐릭터 카드와 `.charx` 포멧에 대해
두꺼운 (아직은 얇은...) 추상화 계층을 제공합니다.

**동일한 포멧을 모든 곳에서 동일하게 작동**시킬
수 있도록 돕는 것이 목표입니다.

## 기능

**(역)직렬화/검증**:

- [x] 전체 표준 준수
  - [캐릭터 카드 v1/v2 명세](https://github.com/malfoyslastname/character-card-spec-v2)
  - [캐릭터 카드 v3 명세](https://github.com/kwaroran/character-card-spec-v3)
- [x] 컨텐츠 파싱
  - [x] 데코레이터 파서
  - [x] 중괄호-구문 (CBS) 파서
- [x] 비표준 필드 호환성
  - [x] RisuAI (*testing...*)

**평가(Evaluation)**:

- [ ] 컨텐츠 평가
  - [ ] 데코레이터 평가기
  - [ ] CBS 평가기
- [ ] 비표준 확장 호환성
  - [ ] RisuAI

**패키징**:

- [x] `.charx`에서 내용 추출
- [ ] `.charx`로 내용 압축

## 사용예

`chara_card`를 `Cargo.toml`의 종속성 목록에 추가하십시오.

**`card.json` 파싱**:

```rust
use chara_card::raw::CharacterCard;

fn parse_card_json(card_json: &str) {
    // Parse character card from JSON string
    let parsed: CharacterCard = serde_json::from_str(card_json).unwrap();
    
    println!("{:#?}", parsed);
}
```

**`.charx`에서 추출**:

> [!WARNING]
> 현재 `charx` 모듈은 무르익지 않았으며, API는 변화할 수 있습니다.

```rust
use chara_card::charx::CharX;

fn parse_charx(bytes: &[u8]) {
    // Extract from archived bytes (.charx)
    let reader = Cursor::new(bytes);
    let _charx = CharX::from(reader).unwrap();
}
```

## 라이선스

[아파치, 버전 2.0](LICENSE)로 라이선스되었습니다.
