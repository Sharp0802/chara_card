use crate::raw::ext::macros::extensions;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Extensions(Vec<Extension>);

impl Deref for Extensions {
    type Target = [Extension];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

extensions! {
    pngExif(Value),
}
