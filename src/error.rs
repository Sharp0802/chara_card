use crate::raw::Version;
use thiserror::Error;
use uriparse::URIError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("version string is malformed")]
    MalformedVersion,
    #[error("URI string is malformed ({0})")]
    MalformedURI(#[from] URIError),
    #[error("unsupported version ({0})")]
    UnsupportedVersion(Version),
    #[error("invalid version ({1}) for specification ({0})")]
    InvalidVersion(String, Version),
    #[error("feature required for version ({0}) is missing")]
    MissingFeature(Version),
    #[error("specified asset name ({0}) is already occupied")]
    AssetNameConflict(String),
}
