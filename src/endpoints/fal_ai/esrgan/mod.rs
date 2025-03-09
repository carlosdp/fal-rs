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

/// Upscale Images
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn esrgan(params: UpscaleInput) -> FalRequest<UpscaleInput, UpscaleOutput> {
    FalRequest::new("fal-ai/esrgan", params)
}
