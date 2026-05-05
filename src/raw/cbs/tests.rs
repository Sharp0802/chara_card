use crate::raw::cbs::{parse, Node};

#[test]
fn test_plain_text() {
    let source = "Hello world!";
    let nodes = parse(source, 0).unwrap();

    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0], Node::Text(0..12));
}

#[test]
fn test_simple_macro() {
    // "user" is at indices 2..6
    let source = "{{user}}";
    let nodes = parse(source, 0).unwrap();

    assert_eq!(nodes.len(), 1);
    assert_eq!(
        nodes[0],
        Node::Macro {
            name: 2..6,
            args: vec![],
        }
    );
}

#[test]
fn test_macro_with_args() {
    // "time" is 2..6, "YYYY" is 8..12, "HH" is 14..16
    let source = "{{time::YYYY::HH}}";
    let nodes = parse(source, 0).unwrap();

    assert_eq!(nodes.len(), 1);
    assert_eq!(
        nodes[0],
        Node::Macro {
            name: 2..6,
            args: vec![vec![Node::Text(8..12)], vec![Node::Text(14..16)],],
        }
    );
}

#[test]
fn test_math_syntax() {
    // " 5+5 " is inside the math block, from index 3..8
    let source = "{{? 5+5 }}";
    let nodes = parse(source, 0).unwrap();

    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0], Node::Math(vec![Node::Text(3..8)]));
}

#[test]
fn test_block_syntax() {
    // name "if" is 3..5
    // arg "1" is 6..7
    // body "Hello" is 9..14
    let source = "{{#if 1}}Hello{{/if}}";
    let nodes = parse(source, 0).unwrap();

    assert_eq!(nodes.len(), 1);
    assert_eq!(
        nodes[0],
        Node::Block {
            name: 3..5,
            args: vec![vec![Node::Text(6..7)]],
            children: vec![Node::Text(9..14)],
        }
    );
}

#[test]
fn test_block_syntax_short_close() {
    // Testing that `{{/}}` works just as well as `{{/if}}`
    let source = "{{#if 1}}Hello{{/}}";
    let nodes = parse(source, 0).unwrap();

    assert_eq!(nodes.len(), 1);
    assert_eq!(
        nodes[0],
        Node::Block {
            name: 3..5,
            args: vec![vec![Node::Text(6..7)]],
            children: vec![Node::Text(9..14)],
        }
    );
}

#[test]
fn test_nested_syntaxes() {
    // A macro inside a math block inside a text stream
    let source = "Score: {{? {{getvar::A}} + 1 }} points";
    let nodes = parse(source, 0).unwrap();

    assert_eq!(nodes.len(), 3);

    // "Score: "
    assert_eq!(nodes[0], Node::Text(0..7));

    // " points"
    assert_eq!(nodes[2], Node::Text(31..38));

    // Math block
    if let Node::Math(math_children) = &nodes[1] {
        assert_eq!(math_children.len(), 3);

        // " "
        assert_eq!(math_children[0], Node::Text(10..11));

        // {{getvar::A}}
        assert_eq!(
            math_children[1],
            Node::Macro {
                name: 13..19,
                args: vec![vec![Node::Text(21..22)]]
            }
        );

        // " + 1 "
        assert_eq!(math_children[2], Node::Text(24..29));
    } else {
        panic!("Expected Math node at index 1");
    }
}

#[test]
fn test_fallback_unclosed_brackets() {
    // The parser should treat broken syntax as pure text
    // and optimize it into a single Text node.
    let source = "Hello {{user and some other text";
    let nodes = parse(source, 0).unwrap();

    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0], Node::Text(0..32));
}

#[test]
fn test_complex_array_spread() {
    // Validating the spec example: {{random::{{spread::{{array::A::B}}}}}}
    let source = "{{random::{{spread::{{array::A::B}}}}}}";
    let nodes = parse(source, 0).unwrap();

    assert_eq!(nodes.len(), 1);

    // Drill down to ensure deep nesting parsed correctly
    let random_args = match &nodes[0] {
        Node::Macro { name, args } if name == &(2..8) => args,
        _ => panic!("Expected random macro"),
    };

    let spread_args = match &random_args[0][0] {
        Node::Macro { name, args } if name == &(12..18) => args,
        _ => panic!("Expected spread macro"),
    };

    let array_args = match &spread_args[0][0] {
        Node::Macro { name, args } if name == &(22..27) => args,
        _ => panic!("Expected array macro"),
    };

    assert_eq!(array_args.len(), 2);
    assert_eq!(array_args[0][0], Node::Text(29..30)); // A
    assert_eq!(array_args[1][0], Node::Text(32..33)); // B
}
