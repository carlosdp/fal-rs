#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpscaleOutput {
    /// The upscaled image.
    pub image: Image,
}

/// Recraft Clarity Upscale
///
/// Category: image-to-image
///
///
///
/// Enhances a given raster image using 'clarity upscale' tool,
/// increasing image resolution, making the image sharper and cleaner.
pub fn recraft_clarity_upscale(params: UpscaleInput) -> FalRequest<UpscaleInput, UpscaleOutput> {
    FalRequest::new("fal-ai/recraft-clarity-upscale", params)
}
