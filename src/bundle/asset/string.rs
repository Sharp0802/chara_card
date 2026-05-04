use std::collections::HashMap;
use isolang::Language;

#[derive(Debug, Clone)]
pub enum TextContent {
    Invariant(String),
    Multilingual(HashMap<Language, String>),
}

impl From<String> for TextContent {
    fn from(value: String) -> Self {
        Self::Invariant(value)
    }
}

impl From<HashMap<Language, String>> for TextContent {
    fn from(value: HashMap<Language, String>) -> Self {
        Self::Multilingual(value)
    }
}

fn infer_language<'a>(set: impl Iterator<Item = &'a TextContent>) -> Language {
    let mut counter = HashMap::new();

    for text in set {
        let TextContent::Multilingual(map) = text else {
            continue;
        };

        for &locale in map.keys() {
            *counter.entry(locale).or_insert(0) += 1;
        }
    }

    counter
        .into_iter()
        .max_by_key(|(_, k)| *k)
        .map(|(l, _)| l)
        .unwrap_or(Language::Eng)
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Text(usize);

pub struct TextBundleBuilder {
    texts: Vec<TextContent>,
}

impl TextBundleBuilder {
    pub fn new() -> Self {
        Self { texts: Vec::new() }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.texts.reserve(additional);
    }

    pub fn push(&mut self, text: TextContent) -> Text {
        self.texts.push(text);
        Text(self.texts.len())
    }

    pub fn build(self) -> TextBundle {
        self.texts.into()
    }
}

#[derive(Debug, Clone)]
pub struct TextBundle {
    texts: Vec<TextContent>,
    default_language: Language,
}

impl From<Vec<TextContent>> for TextBundle {
    fn from(value: Vec<TextContent>) -> Self {
        let default_language = infer_language(value.iter());

        Self {
            texts: value,
            default_language,
        }
    }
}
