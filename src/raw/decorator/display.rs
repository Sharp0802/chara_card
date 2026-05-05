use crate::raw::decorator::Decorator;
use std::fmt::{Display, Formatter};

pub struct DecoratorDisplay<'a> {
    base: &'a str,
    decorator: &'a Decorator,
}

impl<'a> DecoratorDisplay<'a> {
    pub fn new(base: &'a str, decorator: &'a Decorator) -> Self {
        Self { base, decorator }
    }
}

impl<'a> Display for DecoratorDisplay<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.decorator.fmt_with_depth(self.base, f, 0)
    }
}

pub struct DecoratorVecDisplay<'a> {
    base: &'a str,
    decorators: &'a [Decorator],
}

impl<'a> DecoratorVecDisplay<'a> {
    pub fn new(base: &'a str, decorators: &'a [Decorator]) -> Self {
        Self { base, decorators }
    }
}

impl<'a> Display for DecoratorVecDisplay<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for decorator in self.decorators {
            write!(f, "{}", DecoratorDisplay::new(self.base, decorator))?;
        }
        Ok(())
    }
}
