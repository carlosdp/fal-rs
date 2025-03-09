#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_stable-cascade",
    feature = "endpoints_fal-ai_stable-cascade_sote-diffusion"
))]
pub mod sote_diffusion;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// Whether the generated images contain NSFW concepts.
    pub has_nsfw_concepts: Vec<bool>,
    /// The generated image files info.
    pub images: Vec<Image>,
    /// The prompt used for generating the image.
    pub prompt: String,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
    pub timings: Timings,
}

/// Stable Cascade
///
/// Category: text-to-image
/// Machine Type: A100
/// License Type: research
pub fn stable_cascade(params: StableCascadeInput) -> FalRequest<StableCascadeInput, Output> {
    FalRequest::new("fal-ai/stable-cascade", params)
}
