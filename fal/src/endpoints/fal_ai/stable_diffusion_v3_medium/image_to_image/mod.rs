#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

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
pub fn image_to_image(params: ImageToImageInput) -> FalRequest<ImageToImageInput, SD3Output> {
    FalRequest::new("fal-ai/stable-diffusion-v3-medium", params)
}
