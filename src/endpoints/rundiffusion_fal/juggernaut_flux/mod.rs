#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_rundiffusion-fal",
    feature = "endpoints_rundiffusion-fal_juggernaut-flux",
    feature = "endpoints_rundiffusion-fal_juggernaut-flux_base"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_rundiffusion-fal",
        feature = "endpoints_rundiffusion-fal_juggernaut-flux",
        feature = "endpoints_rundiffusion-fal_juggernaut-flux_base"
    )))
)]
pub mod base;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_rundiffusion-fal",
    feature = "endpoints_rundiffusion-fal_juggernaut-flux",
    feature = "endpoints_rundiffusion-fal_juggernaut-flux_lightning"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_rundiffusion-fal",
        feature = "endpoints_rundiffusion-fal_juggernaut-flux",
        feature = "endpoints_rundiffusion-fal_juggernaut-flux_lightning"
    )))
)]
pub mod lightning;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_rundiffusion-fal",
    feature = "endpoints_rundiffusion-fal_juggernaut-flux",
    feature = "endpoints_rundiffusion-fal_juggernaut-flux_pro"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_rundiffusion-fal",
        feature = "endpoints_rundiffusion-fal_juggernaut-flux",
        feature = "endpoints_rundiffusion-fal_juggernaut-flux_pro"
    )))
)]
pub mod pro;

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
/// FLUX.1 [dev], next generation text-to-image model.
pub fn juggernaut_flux(params: DevTextToImageInput) -> FalRequest<DevTextToImageInput, Output> {
    FalRequest::new("rundiffusion-fal/juggernaut-flux", params)
}
