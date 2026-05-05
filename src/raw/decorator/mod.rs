use crate::raw::decorator::macros::decorators;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

mod error;
mod macros;
mod parse;
mod display;

#[cfg(test)]
mod tests;

pub use error::Error;
pub use parse::extract;
use crate::raw::decorator::display::DecoratorDisplay;

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

#[derive(Debug, Clone)]
pub struct Decorator {
    kind: DecoratorKind,
    pub(crate) fallbacks: Vec<Decorator>,
}

impl Decorator {
    pub fn new(kind: DecoratorKind) -> Self {
        Self {
            kind,
            fallbacks: Vec::new(),
        }
    }
    
    pub fn display<'a>(&'a self, base: &'a str) -> DecoratorDisplay<'a> {
        DecoratorDisplay::new(base, self)
    }

    fn fmt_with_depth(&self, text: &str, f: &mut Formatter<'_>, depth: usize) -> std::fmt::Result {
        for _ in 0..(depth + 2) {
            f.write_char('@')?;
        }

        write!(f, "{}\n", self.kind.to_string(text))?;

        for fallback in &self.fallbacks {
            fallback.fmt_with_depth(text, f, depth + 1)?;
        }

        Ok(())
    }
}

decorators! {
    ActivateOnlyAfter { count: number },
    ActivateOnlyEvery { count: number },
    KeepActivateAfterMatch,
    DontActivateAfterMatch,
    Depth { index: number },
    InstructDepth { token_index: number },
    ReverseDepth { index: number },
    ReverseInstructDepth { token_index: number },
    Role { role: Role },
}
