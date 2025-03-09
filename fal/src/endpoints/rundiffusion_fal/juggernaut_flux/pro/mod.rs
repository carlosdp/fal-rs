#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

pub mod image_to_image;
pub mod redux;

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

/// Juggernaut Flux Base
///
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
///
/// Finetuned version of FLUX.1 [dev], for superior text-to-image generation.
pub fn pro(params: DevTextToImageInput) -> FalRequest<DevTextToImageInput, Output> {
    FalRequest::new("rundiffusion-fal/juggernaut-flux/pro", params)
}
