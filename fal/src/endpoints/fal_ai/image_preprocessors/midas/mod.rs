#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MiDaSOutput {
    /// Image with MiDaS depth map
    pub depth_map: Image,
    /// Image with MiDaS normal map
    pub normal_map: Image,
}

/// Image Preprocessors
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn midas(params: MiDaSInput) -> FalRequest<MiDaSInput, MiDaSOutput> {
    FalRequest::new("fal-ai/image-preprocessors/depth-anything/v2", params)
}
