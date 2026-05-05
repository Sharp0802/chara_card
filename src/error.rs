use thiserror::Error;
use crate::raw::Version;

#[derive(Debug, Error)]
pub enum Error {
    #[error("version string is malformed")]
    MalformedVersion,

    #[error("version '{0}' is not supported")]
    UnsupportedVersion(Version),

    #[error("specification '{0}' is not supported")]
    UnsupportedSpec(String),

    #[error("feature ('{1}') to satisfy version (>= '{0}') requirement is missing")]
    MissingFeature(Version, String),

    #[error("asset name '{0}' is duplicated")]
    AssetNameConflict(String),
}
