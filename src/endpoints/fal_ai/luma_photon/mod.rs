#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_luma-photon",
    feature = "endpoints_fal-ai_luma-photon_flash"
))]
pub mod flash;

#[derive(Debug, Serialize, Deserialize)]
pub struct T2IOutput {
    /// The generated image
    pub images: Vec<File>,
}

/// Luma Photon
///
/// Category: text-to-image
/// Machine Type: A100
pub fn luma_photon(params: TextToImageRequest) -> FalRequest<TextToImageRequest, T2IOutput> {
    FalRequest::new("fal-ai/luma-photon", params)
}
