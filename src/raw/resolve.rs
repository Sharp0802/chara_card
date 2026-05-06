use std::borrow::Cow;

/// Represents generic way to resolve given struct-specific index into string.
///
/// It assumes that the given type `T` implements Copy or Clone,
/// and that cloning is sufficiently lightweight.
pub trait Resolve<T> {
    /// Resolves given index into string.
    fn resolve(&'_ self, value: T) -> Cow<'_, str>;
}
