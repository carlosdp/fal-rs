#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux-pro",
    feature = "endpoints_fal-ai_flux-pro_finetuned"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux-pro",
        feature = "endpoints_fal-ai_flux-pro_finetuned"
    )))
)]
pub mod finetuned;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux-pro",
    feature = "endpoints_fal-ai_flux-pro_new"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux-pro",
        feature = "endpoints_fal-ai_flux-pro_new"
    )))
)]
pub mod new;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux-pro",
    feature = "endpoints_fal-ai_flux-pro_v1"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux-pro",
        feature = "endpoints_fal-ai_flux-pro_v1"
    )))
)]
pub mod v1;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux-pro",
    feature = "endpoints_fal-ai_flux-pro_v1-1"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux-pro",
        feature = "endpoints_fal-ai_flux-pro_v1-1"
    )))
)]
pub mod v1_1;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux-pro",
    feature = "endpoints_fal-ai_flux-pro_v1-1-ultra"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux-pro",
        feature = "endpoints_fal-ai_flux-pro_v1-1-ultra"
    )))
)]
pub mod v1_1_ultra;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux-pro",
    feature = "endpoints_fal-ai_flux-pro_v1-1-ultra-finetuned"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux-pro",
        feature = "endpoints_fal-ai_flux-pro_v1-1-ultra-finetuned"
    )))
)]
pub mod v1_1_ultra_finetuned;

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

/// FLUX1.1 [pro] ultra
///
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
///
/// FLUX.1 [pro], next generation text-to-image model.
///
/// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
pub fn flux_pro(params: FluxProTextToImageInput) -> FalRequest<FluxProTextToImageInput, Output> {
    FalRequest::new("fal-ai/flux-pro", params)
}
