use std::borrow::Cow;
use std::ops::Range;
use crate::raw::cbs;
use crate::raw::cbs::Node;
use crate::raw::decorator;
use crate::raw::decorator::Decorator;
use thiserror::Error;
use crate::raw::resolve::Resolve;

#[derive(Debug, Clone)]
pub struct Content {
    content: String,
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

impl Content {
    pub fn decorators(&self) -> &[Decorator] {
        &self.decorators
    }

    pub fn parts(&self) -> &[Node] {
        &self.parts
    }
}

impl Resolve<Range<usize>> for Content {
    fn resolve(&'_ self, value: Range<usize>) -> Cow<'_, str> {
        Cow::Borrowed(&self.as_ref()[value])
    }
}

impl AsRef<str> for Content {
    fn as_ref(&self) -> &str {
        &self.content
    }
}

impl TryFrom<String> for Content {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (range, decorators) = decorator::extract(&value)?;
        let offset = range.start;
        let parts = cbs::parse(&value[offset..], offset).map_err(Error::CBS)?;

        Ok(Self {
            content: value,
            decorators,
            parts,
        })
    }
}
