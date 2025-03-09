#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct AMTInterpolationOutput {
    /// Generated video
    pub video: File,
}

/// AMT Interpolation
///
/// Category: video-to-video
/// Machine Type: A6000
pub fn frame_interpolation(
    params: AMTFrameInterpolationInput,
) -> FalRequest<AMTFrameInterpolationInput, AMTInterpolationOutput> {
    FalRequest::new("fal-ai/amt-interpolation", params)
}
