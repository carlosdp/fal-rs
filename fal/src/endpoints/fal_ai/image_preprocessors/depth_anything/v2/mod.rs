#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct DepthAnythingV2Output {
    /// Image with depth map
    pub image: Image,
}

/// Image Preprocessors
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn v2(params: DepthAnythingV2Input) -> FalRequest<DepthAnythingV2Input, DepthAnythingV2Output> {
    FalRequest::new("fal-ai/image-preprocessors/depth-anything/v2", params)
}
