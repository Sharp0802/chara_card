use thiserror::Error;
use crate::bundle::version::Version;

#[derive(Debug, Error)]
pub enum Error {
    #[error("version string is malformed")]
    MalformedVersion,
    #[error("unsupported version ({0})")]
    UnsupportedVersion(Version),
    #[error("invalid version ({1}) for specification ({0})")]
    InvalidVersion(String, Version),
    #[error("feature required for version ({0}) is missing")]
    MissingFeature(Version),
}
