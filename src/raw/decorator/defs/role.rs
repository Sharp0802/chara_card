use crate::raw::decorator::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Role {
    Assistant,
    System,
    User,
}

impl FromStr for Role {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "assistant" => Ok(Self::Assistant),
            "system" => Ok(Self::System),
            "user" => Ok(Self::User),
            _ => Err(Error::ExpectedRole),
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Assistant => "assistant",
                Self::System => "system",
                Self::User => "user",
            }
        )
    }
}
