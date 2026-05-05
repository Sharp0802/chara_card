use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("version string is malformed")]
    MalformedVersion,
}
