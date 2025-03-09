#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimateDiffV2VOutput {
    /// Seed used for generating the video.
    pub seed: i64,
    /// Generated video file.
    /// {"url":"https://fal-cdn.batuhan-941.workers.dev/files/koala/5Cb_6P_s9wW8f8-g9c4yj.mp4"}
    pub video: File,
}

/// AnimateDiff
///
/// Category: text-to-video
/// Machine Type: A100
pub fn video_to_video(
    params: AnimateDiffV2VTurboInput,
) -> FalRequest<AnimateDiffV2VTurboInput, AnimateDiffV2VOutput> {
    FalRequest::new("fal-ai/fast-animatediff/turbo/video-to-video", params)
}
