#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_animatediff-v2v",
    feature = "endpoints_fal-ai_animatediff-v2v_turbo"
))]
pub mod turbo;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimateDiffV2VOutput {
    /// Seed used for generating the video.
    pub seed: i64,
    pub timings: Timings,
    /// Generated video file./// Generated video file./// {"content_type":"video/mp4","url":"https://storage.googleapis.com/falserverless/model_tests/animatediff_v2v/turbo-rocket-output.mp4"}
    pub video: File,
}

/// AnimateDiff Video-to-Video Evolved
///
/// Category: video-to-video
/// Machine Type: A100
pub fn animatediff_v2v(
    params: AnimateDiffV2VInput,
) -> FalRequest<AnimateDiffV2VInput, AnimateDiffV2VOutput> {
    FalRequest::new("fal-ai/animatediff-v2v", params)
}
