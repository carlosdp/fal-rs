#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CCSROutput {
    /// The generated image file info.
    pub image: Image,
    /// The seed used for the generation.
    pub seed: i64,
}

/// CCSR Upscaler
///
/// Category: image-to-image
/// Machine Type: A100
pub fn ccsr(params: CCSRInput) -> FalRequest<CCSRInput, CCSROutput> {
    FalRequest::new("fal-ai/ccsr", params)
}
