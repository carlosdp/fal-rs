#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_recraft-v3",
    feature = "endpoints_fal-ai_recraft-v3_create-style"
))]
pub mod create_style;

#[derive(Debug, Serialize, Deserialize)]
pub struct TextToImageOutput {
    pub images: Vec<File>,
}

/// Recraft V3
///
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
///
/// State of the art Recraft V3 model by recraft.ai, use it as an API directly through fal.
pub fn recraft_v3(params: TextToImageInput) -> FalRequest<TextToImageInput, TextToImageOutput> {
    FalRequest::new("fal-ai/recraft-v3", params)
}
