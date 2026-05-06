use std::string::FromUtf8Error;
use thiserror::Error;
use zip::result::ZipError;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Zip(#[from] ZipError),

    #[error("{0}: {1}")]
    Io(String, std::io::Error),

    #[error("{0}: not found")]
    NotFound(String),

    #[error("card.json: {0}")]
    InvalidCard(#[from] serde_json::error::Error),

    #[error("codepage: {0}")]
    Codepage(#[from] FromUtf8Error),
}
