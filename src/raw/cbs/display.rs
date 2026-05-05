use crate::raw::cbs::Node;
use std::fmt::{Display, Formatter};

pub struct NodeDisplay<'a> {
    base: &'a str,
    node: &'a Node,
}

impl<'a> NodeDisplay<'a> {
    pub fn new(base: &'a str, node: &'a Node) -> Self {
        Self { base, node }
    }
}

impl<'a> Display for NodeDisplay<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.node {
            Node::Text(range) => {
                // Slice the original string using the saved range
                write!(f, "{}", &self.base[range.clone()])
            }

            Node::Macro { name, args } => {
                write!(f, "{{{}", &self.base[name.clone()])?;
                for arg_nodes in args {
                    write!(f, "::")?;
                    for arg_node in arg_nodes {
                        write!(f, "{}", NodeDisplay::new(self.base, arg_node))?;
                    }
                }
                write!(f, "}}")
            }

            Node::Block {
                name,
                args,
                children,
            } => {
                write!(f, "{{#{}", &self.base[name.clone()])?;
                for arg_nodes in args {
                    write!(f, " ")?;
                    for arg_node in arg_nodes {
                        write!(f, "{}", NodeDisplay::new(self.base, arg_node))?;
                    }
                }
                write!(f, "}}")?;

                // Print all block children
                for child in children {
                    write!(f, "{}", NodeDisplay::new(self.base, child))?;
                }

                // Close the block. Using the full {{/name}} format.
                write!(f, "{{/{}}}", &self.base[name.clone()])
            }

            Node::Math(children) => {
                // The space after `?` is captured by the parser as a Text node,
                // so we don't manually inject a space here.
                write!(f, "{{?")?;
                for child in children {
                    write!(f, "{}", NodeDisplay::new(self.base, child))?;
                }
                write!(f, "}}")
            }
        }
    }
}

pub struct NodeVecDisplay<'a> {
    nodes: &'a [Node],
    base: &'a str,
}

impl<'a> NodeVecDisplay<'a> {
    pub fn new(nodes: &'a [Node], base: &'a str) -> Self {
        Self { nodes, base }
    }
}

impl<'a> Display for NodeVecDisplay<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for node in self.nodes {
            write!(f, "{}", NodeDisplay::new(self.base, node))?;
        }
        Ok(())
    }
}
