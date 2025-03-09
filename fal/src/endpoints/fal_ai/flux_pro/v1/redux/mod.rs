use crate::prelude::*;
use serde::{Deserialize, Serialize};

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
/// FLUX1.1 [pro] ultra is the newest version of FLUX1.1 [pro], maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
///
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
///
/// FLUX.1 Redux [pro] API, next generation text-to-image model.
///
/// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
pub fn redux(params: FluxProRedux) -> FalRequest<FluxProRedux, Output> {
    FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
}
