use crate::raw::decorator::display::DecoratorDisplay;
use crate::raw::decorator::macros::decorators;
use crate::raw::decorator::{Error, Role};
use std::fmt::{Formatter, Write};

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
    
    pub fn kind(&self) -> &DecoratorKind {
        &self.kind
    }

    pub fn display<'a>(&'a self, base: &'a str) -> DecoratorDisplay<'a> {
        DecoratorDisplay::new(base, self)
    }

    pub(crate) fn fmt_with_depth(&self, text: &str, f: &mut Formatter<'_>, depth: usize) -> std::fmt::Result {
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
