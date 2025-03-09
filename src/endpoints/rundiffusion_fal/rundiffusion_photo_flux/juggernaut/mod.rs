#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_rundiffusion-fal",
    feature = "endpoints_rundiffusion-fal_rundiffusion-photo-flux",
    feature = "endpoints_rundiffusion-fal_rundiffusion-photo-flux_juggernaut",
    feature = "endpoints_rundiffusion-fal_rundiffusion-photo-flux_juggernaut_image-to-image"
))]
pub mod image_to_image;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_rundiffusion-fal",
    feature = "endpoints_rundiffusion-fal_rundiffusion-photo-flux",
    feature = "endpoints_rundiffusion-fal_rundiffusion-photo-flux_juggernaut",
    feature = "endpoints_rundiffusion-fal_rundiffusion-photo-flux_juggernaut_inpainting"
))]
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

/// Rundiffusion Photo Flux
///
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
///
/// FLUX.1 [dev], next generation text-to-image model.
pub fn juggernaut(params: PhotoLoraT2IInput) -> FalRequest<PhotoLoraT2IInput, Output> {
    FalRequest::new(
        "rundiffusion-fal/rundiffusion-photo-flux/juggernaut",
        params,
    )
}
