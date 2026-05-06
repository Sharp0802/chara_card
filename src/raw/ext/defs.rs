use crate::raw::ext::impls::{DepthPrompt, PngExif, RisuAI};
use crate::raw::ext::macros::extensions;
use std::ops::Deref;

/// Represents an array of application-specific extensions.
#[derive(Debug, Clone, Default)]
pub struct Extensions(Vec<Extension>);

impl Deref for Extensions {
    type Target = [Extension];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

extensions! {
    depth_prompt(DepthPrompt),
    pngExif(PngExif),
    risu_fullWordMatching(bool),
    risuai(RisuAI),
}
