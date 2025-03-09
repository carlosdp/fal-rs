#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SkyreelsI2VResponse {
    /// The seed used for generation/// The seed used for generation/// 42
    pub seed: i64,
    pub video: File,
}

/// Skyreels V1 (Image-to-Video)
///
/// Category: image-to-video
/// Machine Type: H100
///
///
/// Skyreels Image-to-Video API for fast video generation.
pub fn skyreels_i2v(
    params: SkyreelsI2VRequest,
) -> FalRequest<SkyreelsI2VRequest, SkyreelsI2VResponse> {
    FalRequest::new("fal-ai/skyreels-i2v", params)
}
