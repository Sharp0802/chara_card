use crate::raw::cbs::Node;
use crate::raw::decorator;
use crate::raw::decorator::Decorator;
use crate::raw::resolve::Resolve;
use crate::raw::{cbs, Error};
use serdev::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Content {
    content: String,
    decorators: Vec<Decorator>,
    parts: Vec<Node>,
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

impl Serialize for Content {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.as_ref())
    }
}

impl<'de> Deserialize<'de> for Content {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serdev::de::Error;

        let string = String::deserialize(deserializer)?;
        Self::try_from(string).map_err(D::Error::custom)
    }
}
