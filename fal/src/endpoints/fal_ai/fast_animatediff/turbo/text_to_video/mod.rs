#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimateDiffT2VOutput {
    /// Seed used for generating the video.
    pub seed: i64,
    /// Generated video file./// Generated video file./// {"url":"https://fal-cdn.batuhan-941.workers.dev/files/kangaroo/DSrFBOk9XXIplm_kukI4n.mp4"}
    pub video: File,
}

/// AnimateDiff
///
/// Category: text-to-video
/// Machine Type: A100
pub fn text_to_video(
    params: AnimateDiffT2VTurboInput,
) -> FalRequest<AnimateDiffT2VTurboInput, AnimateDiffT2VOutput> {
    FalRequest::new("fal-ai/fast-animatediff/turbo/text-to-video", params)
}
