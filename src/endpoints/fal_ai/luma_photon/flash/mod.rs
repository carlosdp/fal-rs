#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct T2IOutput {
    /// The generated image
    pub images: Vec<File>,
}

/// Luma Photon
///
/// Category: text-to-image
/// Machine Type: A100
pub fn flash(params: TextToImageRequest) -> FalRequest<TextToImageRequest, T2IOutput> {
    FalRequest::new("fal-ai/luma-photon/flash", params)
}
