#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux",
    feature = "endpoints_fal-ai_flux_dev"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux",
        feature = "endpoints_fal-ai_flux_dev"
    )))
)]
pub mod dev;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux",
    feature = "endpoints_fal-ai_flux_schnell"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux",
        feature = "endpoints_fal-ai_flux_schnell"
    )))
)]
pub mod schnell;

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

/// FLUX.1 [dev]
///
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
///
/// FLUX.1 [dev], next generation text-to-image model.
pub fn flux(params: DevTextToImageInput) -> FalRequest<DevTextToImageInput, Output> {
    FalRequest::new("fal-ai/flux", params)
}
