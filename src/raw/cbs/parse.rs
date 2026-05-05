use crate::raw::cbs::Node;
use winnow::combinator::alt;
use winnow::token::{literal, take_while};
use winnow::Parser;

type ParseResult<T> = Result<T, winnow::error::ErrMode<winnow::error::ContextError>>;

#[derive(Clone, Copy, PartialEq)]
enum Context {
    Root,
    MacroArg,
    BlockArg,
    Math,
    BlockBody,
}

#[derive(Clone, Copy)]
struct CbsParser<'a> {
    base: &'a str,
    global_offset: usize,
}

impl<'a> CbsParser<'a> {
    fn offset(&self, slice: &str) -> usize {
        let local_offset = slice.as_ptr() as usize - self.base.as_ptr() as usize;
        local_offset + self.global_offset
    }

    // Instead of returning a closure, we pass `input` directly into the method
    fn parse_nodes(&self, ctx: Context, input: &mut &'a str) -> ParseResult<Vec<Node>> {
        let mut nodes = Vec::new();
        while !input.is_empty() {
            // Context terminators
            if ctx == Context::Root && input.starts_with("{{/") {
                break;
            }
            if ctx == Context::BlockBody && input.starts_with("{{/") {
                break;
            }
            if ctx == Context::MacroArg && (input.starts_with("::") || input.starts_with("}}")) {
                break;
            }
            if ctx == Context::BlockArg && (input.starts_with(' ') || input.starts_with("}}")) {
                break;
            }
            if ctx == Context::Math && input.starts_with("}}") {
                break;
            }

            // Using closures inline inside `alt` satisfies winnow's traits perfectly
            let node = alt((
                |i: &mut &'a str| self.parse_block_or_macro_or_math(i),
                |i: &mut &'a str| self.parse_text(ctx, i),
            ))
            .parse_next(input)?;

            nodes.push(node);
        }
        Ok(nodes)
    }

    fn parse_block_or_macro_or_math(&self, input: &mut &'a str) -> ParseResult<Node> {
        if input.starts_with("{{#") {
            self.parse_block(input)
        } else if input.starts_with("{{?") {
            self.parse_math(input)
        } else if input.starts_with("{{") && !input.starts_with("{{/") {
            self.parse_macro(input)
        } else {
            Err(winnow::error::ErrMode::Backtrack(
                winnow::error::ContextError::new(),
            ))
        }
    }

    fn parse_math(&self, input: &mut &'a str) -> ParseResult<Node> {
        let _ = literal("{{?").parse_next(input)?;
        let nodes = self.parse_nodes(Context::Math, input)?;
        let _ = literal("}}").parse_next(input)?;

        Ok(Node::Math(nodes))
    }

    fn parse_macro(&self, input: &mut &'a str) -> ParseResult<Node> {
        let _ = literal("{{").parse_next(input)?;

        let name_slice = take_while(1.., |c| c != ':' && c != '}' && c != '{').parse_next(input)?;
        let name_start = self.offset(name_slice);
        let name_span = name_start..(name_start + name_slice.len());

        let mut args = Vec::new();
        while input.starts_with("::") {
            let _ = literal("::").parse_next(input)?;
            let arg_nodes = self.parse_nodes(Context::MacroArg, input)?;
            args.push(arg_nodes);
        }

        let _ = literal("}}").parse_next(input)?;

        Ok(Node::Macro {
            name: name_span,
            args,
        })
    }

    fn parse_block(&self, input: &mut &'a str) -> ParseResult<Node> {
        let _ = literal("{{#").parse_next(input)?;

        let name_slice = take_while(1.., |c| c != ' ' && c != '}').parse_next(input)?;
        let name_start = self.offset(name_slice);
        let name_span = name_start..(name_start + name_slice.len());

        let mut args = Vec::new();
        while input.starts_with(' ') {
            let _ = take_while(1.., ' ').parse_next(input)?;
            if input.starts_with("}}") {
                break;
            }
            let arg_nodes = self.parse_nodes(Context::BlockArg, input)?;
            if !arg_nodes.is_empty() {
                args.push(arg_nodes);
            }
        }

        let _ = literal("}}").parse_next(input)?;
        let children = self.parse_nodes(Context::BlockBody, input)?;

        let _ = literal("{{/").parse_next(input)?;
        let _ = take_while(0.., |c| c != '}').parse_next(input)?;
        let _ = literal("}}").parse_next(input)?;

        Ok(Node::Block {
            name: name_span,
            args,
            children,
        })
    }

    fn parse_text(&self, ctx: Context, input: &mut &'a str) -> ParseResult<Node> {
        let mut bytes_consumed = 0;

        while bytes_consumed < input.len() {
            let tail = &input[bytes_consumed..];

            if bytes_consumed > 0 && tail.starts_with("{{") {
                break;
            }
            if ctx == Context::MacroArg && (tail.starts_with("::") || tail.starts_with("}}")) {
                break;
            }
            if ctx == Context::BlockArg && (tail.starts_with(' ') || tail.starts_with("}}")) {
                break;
            }
            if ctx == Context::Math && tail.starts_with("}}") {
                break;
            }
            if ctx == Context::Root && tail.starts_with("{{/") {
                break;
            }
            if ctx == Context::BlockBody && tail.starts_with("{{/") {
                break;
            }

            let c = tail.chars().next().unwrap();
            bytes_consumed += c.len_utf8();
        }

        if bytes_consumed == 0 {
            return Err(winnow::error::ErrMode::Backtrack(
                winnow::error::ContextError::new(),
            ));
        }

        let text_slice = &input[..bytes_consumed];
        *input = &input[bytes_consumed..];

        let start = self.offset(text_slice);

        Ok(Node::Text(start..(start + text_slice.len())))
    }
}

fn optimize_nodes(nodes: &mut Vec<Node>) {
    let mut i = 0;
    while i < nodes.len() {
        match &mut nodes[i] {
            Node::Macro { args, .. } => {
                for arg in args {
                    optimize_nodes(arg);
                }
            }
            Node::Block { args, children, .. } => {
                for arg in args {
                    optimize_nodes(arg);
                }
                optimize_nodes(children);
            }
            Node::Math(children) => optimize_nodes(children),
            Node::Text(_) => {}
        }

        if i > 0 {
            if let (Node::Text(prev), Node::Text(curr)) = (&nodes[i - 1], &nodes[i]) {
                if prev.end == curr.start {
                    nodes[i - 1] = Node::Text(prev.start..curr.end);
                    nodes.remove(i);
                    continue;
                }
            }
        }
        i += 1;
    }
}

pub fn parse<'a>(content: &'a str, global_offset: usize) -> Result<Vec<Node>, String> {
    let parser = CbsParser {
        base: content,
        global_offset,
    };

    let mut root_parser = |i: &mut &'a str| parser.parse_nodes(Context::Root, i);

    match root_parser.parse(content) {
        Ok(mut nodes) => {
            optimize_nodes(&mut nodes);
            Ok(nodes)
        }
        Err(e) => Err(format!("Parse error:\n{}", e)),
    }
}
