#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux-general",
    feature = "endpoints_fal-ai_flux-general_differential-diffusion"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux-general",
        feature = "endpoints_fal-ai_flux-general_differential-diffusion"
    )))
)]
pub mod differential_diffusion;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux-general",
    feature = "endpoints_fal-ai_flux-general_image-to-image"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux-general",
        feature = "endpoints_fal-ai_flux-general_image-to-image"
    )))
)]
pub mod image_to_image;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux-general",
    feature = "endpoints_fal-ai_flux-general_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux-general",
        feature = "endpoints_fal-ai_flux-general_inpainting"
    )))
)]
pub mod inpainting;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux-general",
    feature = "endpoints_fal-ai_flux-general_rf-inversion"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux-general",
        feature = "endpoints_fal-ai_flux-general_rf-inversion"
    )))
)]
pub mod rf_inversion;

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

/// FLUX.1 [dev] with Controlnets and Loras
///
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
///
/// FLUX.1 [dev], next generation text-to-image model.
pub fn flux_general(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
    FalRequest::new("fal-ai/flux-general", params)
}
