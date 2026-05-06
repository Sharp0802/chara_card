use crate::charx::asset::Asset;
use crate::charx::source::Source;
use crate::charx::Error;
use crate::raw::NestedCharacterCard;
use std::collections::HashMap;
use std::io::{Read, Seek};
use url::Url;
use zip::result::ZipError;
use zip::ZipArchive;

pub struct CharX {
    pub card: NestedCharacterCard,
    pub assets: HashMap<String, Vec<Asset>>,
}

impl<T: Read + Seek> TryFrom<CharXImport<T>> for CharX {
    type Error = Error;

    fn try_from(mut value: CharXImport<T>) -> Result<Self, Self::Error> {
        let mut assets = HashMap::new();
        for raw in value
            .card
            .data
            .v3
            .as_ref()
            .map(|v| v.assets.as_slice())
            .unwrap_or_default()
        {
            let source: Source = raw.uri.clone().into();
            let asset = Asset::try_from(source, &raw.ext, &mut value.archive)?;

            assets
                .entry(raw.name.clone())
                .or_insert_with(Vec::new)
                .push(asset);
        }

        Ok(Self {
            card: value.card,
            assets,
        })
    }
}

pub struct CharXImport<T: Read + Seek> {
    card: NestedCharacterCard,
    archive: ZipArchive<T>,
}

impl<T: Read + Seek> CharXImport<T> {
    pub fn from_reader(reader: T) -> Result<Self, Error> {
        let mut archive = ZipArchive::new(reader)?;

        let file = match archive.by_name("card.json") {
            Ok(file) => file,
            Err(ZipError::FileNotFound) => return Err(Error::NotFound("card.json".into())),
            Err(e) => return Err(e.into()),
        };

        let card: NestedCharacterCard = serde_json::from_reader(file)?;

        Ok(Self {
            card: card,
            archive,
        })
    }

    pub fn externals(&self) -> impl Iterator<Item = &Url> {
        self.card
            .data
            .v3
            .as_ref()
            .map(|v| v.assets.iter())
            .unwrap_or_default()
            .map(|a| &a.uri)
            .filter(|u| u.scheme() != "embeded" && u.scheme() != "ccdefault")
    }
}
