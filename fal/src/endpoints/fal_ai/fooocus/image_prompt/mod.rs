#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

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
pub fn image_prompt(
    params: FooocusImagePromptInput,
) -> FalRequest<FooocusImagePromptInput, FooocusOutput> {
    FalRequest::new("fal-ai/fooocus/image-prompt", params)
}
