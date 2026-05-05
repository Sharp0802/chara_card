use crate::raw::decorator::Error;
use crate::raw::decorator::{Decorator, DecoratorKind};
use std::iter::Peekable;
use std::str::Lines;

fn parse(text: &str, lines: &mut Peekable<Lines<'_>>) -> Result<Vec<Decorator>, Error> {
    let mut decorators: Vec<Decorator> = Vec::new();

    while let Some(&line) = lines.peek() {
        if !line.starts_with("@@") {
            break;
        }
        let line = lines.next().unwrap();

        let (line, depth) = {
            let begin = line.len();
            let line = line.trim_start_matches("@");
            (line, begin - line.len() - 2)
        };

        let mut cursor = &mut decorators;
        let mut remain = depth;
        while remain > 0 {
            let Some(next) = cursor.last_mut() else {
                return Err(Error::InvalidDepth);
            };

            cursor = &mut next.fallbacks;
            remain -= 1;
        }

        // Pass the base text into parse_line
        let decorator = DecoratorKind::parse_line(text, line)?;
        cursor.push(Decorator::new(decorator));
    }

    Ok(decorators)
}

pub fn extract(text: &str) -> Result<(std::ops::Range<usize>, Vec<Decorator>), Error> {
    let mut lines = text.lines().peekable();

    let decorators = parse(text, &mut lines)?;

    let content_range = if let Some(&first_line) = lines.peek() {
        let start_offset = first_line.as_ptr() as usize - text.as_ptr() as usize;
        start_offset..text.len()
    } else {
        0..0
    };

    Ok((content_range, decorators))
}
