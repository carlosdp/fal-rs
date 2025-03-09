#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// The URL of the generated image.
    pub image: Image,
    /// The seed used to generate the image.
    pub seed: i64,
}

/// Layer Diffusion XL
///
/// Category: text-to-image
/// Machine Type: A100
pub fn layer_diffusion(params: Input) -> FalRequest<Input, Output> {
    FalRequest::new("fal-ai/layer-diffusion", params)
}
