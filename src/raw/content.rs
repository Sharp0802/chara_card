use crate::raw::cbs;
use crate::raw::cbs::Node;
use crate::raw::decorator;
use crate::raw::decorator::Decorator;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Content {
    content: String,
    decorator_size: usize,
    decorators: Vec<Decorator>,
    parts: Vec<Node>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Decorator(#[from] decorator::Error),

    #[error("{0}")]
    CBS(String),
}

impl TryFrom<String> for Content {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (range, decorators) = decorator::extract(&value)?;
        let offset = range.start;
        let parts = cbs::parse(&value[offset..], offset).map_err(Error::CBS)?;

        Ok(Self {
            content: value,
            decorator_size: offset,
            decorators,
            parts,
        })
    }
}


