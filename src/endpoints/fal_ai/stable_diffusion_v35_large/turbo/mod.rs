#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_stable-diffusion-v35-large",
    feature = "endpoints_fal-ai_stable-diffusion-v35-large_turbo",
    feature = "endpoints_fal-ai_stable-diffusion-v35-large_turbo_image-to-image"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_stable-diffusion-v35-large",
        feature = "endpoints_fal-ai_stable-diffusion-v35-large_turbo",
        feature = "endpoints_fal-ai_stable-diffusion-v35-large_turbo_image-to-image"
    )))
)]
pub mod image_to_image;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_stable-diffusion-v35-large",
    feature = "endpoints_fal-ai_stable-diffusion-v35-large_turbo",
    feature = "endpoints_fal-ai_stable-diffusion-v35-large_turbo_inpaint"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_stable-diffusion-v35-large",
        feature = "endpoints_fal-ai_stable-diffusion-v35-large_turbo",
        feature = "endpoints_fal-ai_stable-diffusion-v35-large_turbo_inpaint"
    )))
)]
pub mod inpaint;

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

/// Stable Diffusion 3.5 Large
///
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
pub fn turbo(params: TextToImageTurboInput) -> FalRequest<TextToImageTurboInput, Output> {
    FalRequest::new("fal-ai/stable-diffusion-v35-large/turbo", params)
}
