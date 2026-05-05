use crate::raw::cbs::display::NodeDisplay;
use std::ops::Range;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Node {
    Text(Range<usize>),

    Macro {
        name: Range<usize>,
        args: Vec<Vec<Node>>,
    },

    Block {
        name: Range<usize>,
        args: Vec<Vec<Node>>,
        children: Vec<Node>,
    },

    Math(Vec<Node>),
}

impl Node {
    pub fn display<'a>(&'a self, base: &'a str) -> NodeDisplay<'a> {
        NodeDisplay::new(base, self)
    }
}
