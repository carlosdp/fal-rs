#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_fooocus",
    feature = "endpoints_fal-ai_fooocus_image-prompt"
))]
pub mod image_prompt;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_fooocus",
    feature = "endpoints_fal-ai_fooocus_inpaint"
))]
pub mod inpaint;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_fooocus",
    feature = "endpoints_fal-ai_fooocus_upscale-or-vary"
))]
pub mod upscale_or_vary;

#[derive(Debug, Serialize, Deserialize)]
pub struct FooocusOutput {
    /// Whether the generated images contain NSFW concepts.
    pub has_nsfw_concepts: Vec<bool>,
    /// The generated image file info.
    pub images: Vec<Image>,
    /// The time taken for the generation process.
    pub timings: Timings,
}

/// Fooocus
///
/// Category: text-to-image
/// Machine Type: A100
pub fn fooocus(params: FooocusLegacyInput) -> FalRequest<FooocusLegacyInput, FooocusOutput> {
    FalRequest::new("fal-ai/fooocus", params)
}
