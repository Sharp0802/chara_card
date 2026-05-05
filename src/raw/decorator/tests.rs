use crate::raw::decorator::{extract, Error, Role};
use crate::raw::decorator::DecoratorKind;

#[test]
fn test_no_decorators() {
    let text = "Just some standard text\nWithout any decorators.";
    let (content_range, decorators) = extract(text).unwrap();

    assert!(decorators.is_empty());
    assert_eq!(content_range, 0..text.len());
    assert_eq!(&text[content_range], "Just some standard text\nWithout any decorators.");
}

#[test]
fn test_single_decorator() {
    let text = "@@role user\nThe actual content.";
    let (content_range, decorators) = extract(text).unwrap();

    assert_eq!(decorators.len(), 1);
    assert!(matches!(decorators[0].kind, DecoratorKind::Role { role: Role::User }));

    assert_eq!(&text[content_range], "The actual content.");
}

#[test]
fn test_multiple_decorators() {
    let text = "@@role system\n@@activate_only_after 5\nContent starts here.";
    let (content_range, decorators) = extract(text).unwrap();

    assert_eq!(decorators.len(), 2);

    assert!(matches!(decorators[0].kind, DecoratorKind::Role { role: Role::System }));
    assert!(matches!(decorators[1].kind, DecoratorKind::ActivateOnlyAfter { count: 5 }));

    assert_eq!(&text[content_range], "Content starts here.");
}

#[test]
fn test_fallback_depths() {
    // @@ = depth 0
    // @@@ = depth 1 (fallback of the previous)
    // @@@@ = depth 2 (fallback of the fallback)
    let text = "@@role assistant\n@@@depth 2\n@@@@instruct_depth 1\nContent";
    let (content_range, decorators) = extract(text).unwrap();

    assert_eq!(decorators.len(), 1); // Only 1 root decorator
    assert_eq!(&text[content_range], "Content");

    let root = &decorators[0];
    assert!(matches!(root.kind, DecoratorKind::Role { role: Role::Assistant }));
    assert_eq!(root.fallbacks.len(), 1);

    let fallback_1 = &root.fallbacks[0];
    assert!(matches!(fallback_1.kind, DecoratorKind::Depth { index: 2 }));
    assert_eq!(fallback_1.fallbacks.len(), 1);

    let fallback_2 = &fallback_1.fallbacks[0];
    assert!(matches!(fallback_2.kind, DecoratorKind::InstructDepth { token_index: 1 }));
    assert!(fallback_2.fallbacks.is_empty());
}

#[test]
fn test_invalid_depth_error() {
    // Trying to assign a fallback before a root decorator exists
    let text = "@@@depth 2\nContent";
    let result = extract(text);

    assert!(matches!(result, Err(Error::InvalidDepth)));
}

#[test]
fn test_misc_decorator() {
    // "unknown_decorator" is at index 2..19
    // "arg1" is at 20..24
    // "arg2" is at 26..30
    let text = "@@unknown_decorator arg1, arg2\nContent";
    let (content_range, decorators) = extract(text).unwrap();

    assert_eq!(decorators.len(), 1);
    assert_eq!(&text[content_range], "Content");

    if let DecoratorKind::Misc { name, args } = &decorators[0].kind {
        assert_eq!(&text[name.clone()], "unknown_decorator");
        assert_eq!(args.len(), 2);
        assert_eq!(&text[args[0].clone()], "arg1");
        assert_eq!(&text[args[1].clone()], "arg2");
    } else {
        panic!("Expected DecoratorKind::Misc");
    }
}

#[test]
fn test_decorator_parsing_errors() {
    // Missing required argument for "role"
    let err1 = extract("@@role\nContent").unwrap_err();
    assert!(matches!(err1, Error::InsufficientArgument));

    // Invalid role type
    let err2 = extract("@@role god\nContent").unwrap_err();
    assert!(matches!(err2, Error::ExpectedRole));

    // Extraneous arguments for a numeric decorator
    let err3 = extract("@@activate_only_after 5, 10\nContent").unwrap_err();
    assert!(matches!(err3, Error::ExtraneousArgument));

    // Invalid integer
    let err4 = extract("@@activate_only_after text\nContent").unwrap_err();
    assert!(matches!(err4, Error::ExpectedInt(_)));
}
