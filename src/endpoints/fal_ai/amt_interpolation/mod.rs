#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_amt-interpolation",
    feature = "endpoints_fal-ai_amt-interpolation_frame-interpolation"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_amt-interpolation",
        feature = "endpoints_fal-ai_amt-interpolation_frame-interpolation"
    )))
)]
pub mod frame_interpolation;

#[derive(Debug, Serialize, Deserialize)]
pub struct AMTInterpolationOutput {
    /// Generated video
    pub video: File,
}

/// AMT Interpolation
///
/// Category: video-to-video
/// Machine Type: A6000
pub fn amt_interpolation(
    params: AMTInterpolationInput,
) -> FalRequest<AMTInterpolationInput, AMTInterpolationOutput> {
    FalRequest::new("fal-ai/amt-interpolation", params)
}
