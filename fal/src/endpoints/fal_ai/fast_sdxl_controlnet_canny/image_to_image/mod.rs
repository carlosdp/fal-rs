#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// Whether the generated images contain NSFW concepts.
    pub has_nsfw_concepts: Vec<bool>,
    /// The generated image files info.
    pub images: Vec<Image>,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
    pub timings: Timings,
}

/// ControlNet SDXL
///
/// Category: text-to-image
pub fn image_to_image(
    params: ImageToImageControlNetInput,
) -> FalRequest<ImageToImageControlNetInput, Output> {
    FalRequest::new("fal-ai/fast-sdxl-controlnet-canny/image-to-image", params)
}
