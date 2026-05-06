use crate::charx::source::Source;
use crate::charx::Error;
use std::borrow::Cow;
use std::fs::File;
use std::io::{Read, Seek};
use std::ops::Deref;
use zip::ZipArchive;

#[derive(Debug, Clone)]
pub struct Data {
    data: Cow<'static, [u8]>,
    mime: &'static str,
}

impl Data {
    pub fn new(data: Cow<'static, [u8]>, ext: &str) -> Self {
        if let Some(inferred) = infer::get(&data) {
            return Self {
                data,
                mime: inferred.mime_type(),
            };
        }

        let mime = mime_guess::from_ext(ext)
            .first_raw()
            .unwrap_or("application/octet-stream");

        Self { data, mime }
    }
}

impl Deref for Data {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.data.as_ref()
    }
}

impl AsRef<[u8]> for Data {
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct Asset {
    data: Option<Data>,
    source: Source,
}

impl Asset {
    pub fn data(&self) -> Option<&Data> {
        self.data.as_ref()
    }
    
    pub fn source(&self) -> &Source {
        &self.source
    }
    
    pub fn try_from(
        source: Source,
        ext: &str,
        archive: &mut ZipArchive<impl Read + Seek>,
    ) -> Result<Self, Error> {
        Ok(match source {
            Source::CCDefault => Self {
                data: None,
                source: Source::CCDefault,
            },

            Source::Embedded(path) => {
                let mut file = archive.by_path(&path)?;

                let mut buffer = Vec::new();
                if let Err(e) = file.read_to_end(&mut buffer) {
                    return Err(Error::Io(path, e));
                }

                Self {
                    data: Some(Data::new(Cow::Owned(buffer), ext)),
                    source: Source::Embedded(path),
                }
            }

            Source::File(path) => {
                let mut file = match File::open(&path) {
                    Ok(file) => file,
                    Err(e) => return Err(Error::Io(path, e)),
                };

                let mut buffer = Vec::new();
                if let Err(e) = file.read_to_end(&mut buffer) {
                    return Err(Error::Io(path, e));
                }

                Self {
                    data: Some(Data::new(Cow::Owned(buffer), ext)),
                    source: Source::File(path),
                }
            }

            Source::Misc(url) => Self {
                data: None,
                source: Source::Misc(url),
            },
        })
    }
}
