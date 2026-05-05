use std::borrow::Cow;

pub trait Resolve<T> {
    fn resolve(&'_ self, value: T) -> Cow<'_, str>;
}
