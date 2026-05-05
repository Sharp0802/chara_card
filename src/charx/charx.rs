use crate::charx::Error;
use crate::raw::CharacterCard;
use std::collections::HashMap;
use std::io::{BufReader, Read, Seek};
use zip::result::ZipError;
use zip::ZipArchive;

pub struct CharX {
    pub card: CharacterCard,
    pub assets: HashMap<String, Vec<u8>>,
}

impl CharX {
    pub fn from(reader: impl Read + Seek) -> Result<Self, Error> {
        let mut archive = ZipArchive::new(reader)?;

        let file = match archive.by_name("card.json") {
            Ok(file) => file,
            Err(ZipError::FileNotFound) => return Err(Error::NotFound("card.json".into())),
            Err(e) => return Err(e.into()),
        };

        let mut buffer = String::new();
        BufReader::new(file)
            .read_to_string(&mut buffer)
            .map_err(|e| Error::Io("card.json".into(), e))?;

        let card: CharacterCard = serde_json::from_str(&buffer)?;
        let nested = card.to_nested();

        let mut assets = HashMap::new();

        if let Some(v3) = &nested.data.v3 {
            for asset in &v3.assets {
                let buffer = match asset.uri.scheme() {
                    // NOTE: it's not misspelling;
                    //       standard forces to use 'embeded', not 'embedded'.
                    //       but for compatibility, 'embedded' will also be accepted.
                    "embeded" | "embedded" => {
                        let mut path = String::new();

                        if let Some(host) = asset.uri.host_str() {
                            path.push_str(host);
                            path.push('/');
                        }

                        path.push_str(asset.uri.path());

                        let file = match archive.by_name(&path) {
                            Ok(file) => file,
                            Err(ZipError::FileNotFound) => return Err(Error::NotFound(path)),
                            Err(e) => return Err(e.into()),
                        };

                        let mut reader = BufReader::new(file);
                        let mut buffer = Vec::new();
                        reader
                            .read_to_end(&mut buffer)
                            .map_err(|e| Error::Io(path, e))?;

                        buffer
                    }

                    _ => {
                        continue;
                    }
                };

                assets.insert(asset.name.clone(), buffer);
            }
        }

        Ok(Self {
            card: CharacterCard::Nested(nested),
            assets,
        })
    }
}
