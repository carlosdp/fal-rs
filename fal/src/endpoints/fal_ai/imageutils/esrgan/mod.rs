#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpscaleOutput {
    /// Upscaled image
    pub image: Image,
}

/// Midas Depth Estimation
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn esrgan(params: UpscaleInput) -> FalRequest<UpscaleInput, UpscaleOutput> {
    FalRequest::new("fal-ai/imageutils/depth", params)
}
