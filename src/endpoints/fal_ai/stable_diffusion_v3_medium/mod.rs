#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_stable-diffusion-v3-medium",
    feature = "endpoints_fal-ai_stable-diffusion-v3-medium_image-to-image"
))]
pub mod image_to_image;

#[derive(Debug, Serialize, Deserialize)]
pub struct SD3Output {
    /// Whether the generated images contain NSFW concepts.
    pub has_nsfw_concepts: Vec<bool>,
    /// The generated image files info.
    pub images: Vec<Image>,
    /// The number of images generated.
    pub num_images: i64,
    /// The prompt used for generating the image.
    pub prompt: String,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
    pub timings: Timings,
}

/// Stable Diffusion V3
///
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
pub fn stable_diffusion_v3_medium(
    params: TextToImageInput,
) -> FalRequest<TextToImageInput, SD3Output> {
    FalRequest::new("fal-ai/stable-diffusion-v3-medium", params)
}
