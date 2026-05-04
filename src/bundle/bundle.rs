use crate::bundle::asset::string::{Text, TextBundleBuilder};
use crate::bundle::asset::AssetBundle;
use crate::bundle::version::Version;
use crate::bundle::Error;
use crate::raw;
use isolang::Language;
use jiff::Timestamp;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Bundle {
    spec: Version,

    created_at: Option<Timestamp>,
    modified_at: Option<Timestamp>,

    version: Option<String>,
    tags: Vec<String>,
    creator: Vec<String>,
    source: Vec<String>,

    name: Text,
    description: Text,
    scenario: Text,
    notes: Option<Text>,
    greetings: Vec<Text>,
    group_only_greetings: Vec<Text>,
    message_example: Vec<Text>,
    system_prompt: Option<Text>,
    post_history_instructions: Option<Text>,

    assets: AssetBundle,
}

impl Default for Bundle {
    fn default() -> Self {
        Self {
            spec: Version::V3,

            created_at: None,
            modified_at: None,

            version: None,
            tags: vec![],
            creator: vec![],
            source: vec![],

            name: Default::default(),
            description: Default::default(),
            scenario: Default::default(),
            notes: None,
            greetings: vec![],
            group_only_greetings: vec![],
            message_example: vec![],
            system_prompt: None,
            post_history_instructions: None,

            assets: AssetBundle::new(TextBundleBuilder::new().build()),
        }
    }
}

impl Bundle {
    fn apply_v1(&mut self, text_bundle: &mut TextBundleBuilder, v1: raw::v1::CharacterCardData) {
        self.name = text_bundle.push(v1.name.into());
        self.description = text_bundle.push(v1.description.into());
        self.scenario = text_bundle.push(v1.scenario.into());
        self.message_example = v1
            .mes_example
            .split("<START>")
            .map(|s| text_bundle.push(s.trim().to_owned().into()))
            .collect();

        let first_mes = text_bundle.push(v1.first_mes.into());
        self.greetings.push(first_mes);

        // TODO: data.v1.personality;
    }

    fn apply_v2(
        &mut self,
        text_bundle: &mut TextBundleBuilder,
        notes: &mut HashMap<Language, String>,
        v2: raw::v2::CharacterCardData,
    ) {
        let len = v2.alternate_greetings.len();
        text_bundle.reserve(len);
        self.greetings.reserve(len);
        for greeting in v2.alternate_greetings {
            let id = text_bundle.push(greeting.into());
            self.greetings.push(id);
        }

        if let Some(inner) = v2.version2_0.into() {
            self.creator = vec![inner.creator.into()];
            self.tags = inner.tags;
            self.version = Some(inner.character_version);

            // TODO: inner.extensions;
        }

        self.system_prompt = Some(text_bundle.push(v2.system_prompt.into()));
        self.post_history_instructions =
            Some(text_bundle.push(v2.post_history_instructions.into()));

        notes.insert(Language::Eng, v2.creator_notes);

        // TODO: v2.character_book;
    }

    fn apply_v3(&mut self, text_bundle: &mut TextBundleBuilder, notes: &mut HashMap<Language, String>, v3: raw::v3::CharacterCardData) {
        // TODO: v3.assets;
        // TODO: v3.nickname;

        if let Some(creator_notes) = v3.creator_notes_multilingual {
            notes.extend(creator_notes.into_iter());
        }

        self.source = v3.source.unwrap_or_default();

        let len = v3.group_only_greetings.len();
        text_bundle.reserve(len);
        self.greetings.reserve(len);
        for greeting in v3.group_only_greetings {
            let id = text_bundle.push(greeting.into());
            self.greetings.push(id);
        }

        self.created_at = v3.creation_date;
        self.modified_at = v3.modification_date;
    }
}

impl TryFrom<raw::CharacterCard> for Bundle {
    type Error = Error;

    fn try_from(value: raw::CharacterCard) -> Result<Self, Self::Error> {
        let (spec, data) = match value {
            raw::CharacterCard::Nested(nested) => {
                let version: Version = nested.spec_version.parse()?;

                let Some(spec_name) = version.name() else {
                    return Err(Error::UnsupportedVersion(version));
                };

                if spec_name != nested.spec {
                    return Err(Error::InvalidVersion(nested.spec, version));
                }

                (version, nested.data)
            }

            raw::CharacterCard::Flat(v1) => (
                Version::V1,
                raw::CharacterCardData {
                    v1,
                    v2: None.into(),
                    v3: None.into(),
                },
            ),
        };

        let mut bundle = Bundle::default();
        let mut text_bundle = TextBundleBuilder::new();
        let mut notes = HashMap::new();

        bundle.apply_v1(&mut text_bundle, data.v1);

        if let Some(v2) = data.v2.into() {
            bundle.apply_v2(&mut text_bundle, &mut notes, v2);
        } else if spec >= Version::V2 {
            return Err(Error::MissingFeature(Version::V2));
        }

        if let Some(v3) = data.v3.into() {
            bundle.apply_v3(&mut text_bundle, &mut notes, v3);
        } else if spec >= Version::V3 {
            return Err(Error::MissingFeature(Version::V3));
        }

        if notes.len() > 0 {
            bundle.notes = Some(text_bundle.push(notes.into()));
        }

        bundle.assets = AssetBundle::new(text_bundle.build());

        Ok(bundle)
    }
}
