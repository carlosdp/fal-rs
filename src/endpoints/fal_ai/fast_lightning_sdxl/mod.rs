#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_fast-lightning-sdxl",
    feature = "endpoints_fal-ai_fast-lightning-sdxl_image-to-image"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_fast-lightning-sdxl",
        feature = "endpoints_fal-ai_fast-lightning-sdxl_image-to-image"
    )))
)]
pub mod image_to_image;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_fast-lightning-sdxl",
    feature = "endpoints_fal-ai_fast-lightning-sdxl_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_fast-lightning-sdxl",
        feature = "endpoints_fal-ai_fast-lightning-sdxl_inpainting"
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

/// Stable Diffusion XL Lightning
///
/// Category: text-to-image
/// Machine Type: A100
pub fn fast_lightning_sdxl(
    params: TextToImageLightningInput,
) -> FalRequest<TextToImageLightningInput, Output> {
    FalRequest::new("fal-ai/fast-lightning-sdxl", params)
}
