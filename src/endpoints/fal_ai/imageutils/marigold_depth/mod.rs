#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MarigoldDepthMapOutput {
    /// The depth map.
    pub image: Image,
}

/// Midas Depth Estimation
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn marigold_depth(
    params: MarigoldDepthMapInput,
) -> FalRequest<MarigoldDepthMapInput, MarigoldDepthMapOutput> {
    FalRequest::new("fal-ai/imageutils/marigold-depth", params)
}
