#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_sd15-depth-controlnet",
    feature = "endpoints_fal-ai_sd15-depth-controlnet_image-to-image"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_sd15-depth-controlnet",
        feature = "endpoints_fal-ai_sd15-depth-controlnet_image-to-image"
    )))
)]
pub mod image_to_image;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_sd15-depth-controlnet",
    feature = "endpoints_fal-ai_sd15-depth-controlnet_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_sd15-depth-controlnet",
        feature = "endpoints_fal-ai_sd15-depth-controlnet_inpainting"
    )))
)]
pub mod inpainting;

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

/// SD 1.5 Depth ControlNet
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn sd15_depth_controlnet(
    params: TextToImageControlNetInput,
) -> FalRequest<TextToImageControlNetInput, Output> {
    FalRequest::new("fal-ai/sd15-depth-controlnet", params)
}
