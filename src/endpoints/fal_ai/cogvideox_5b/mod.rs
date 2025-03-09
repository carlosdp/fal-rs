#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_cogvideox-5b",
    feature = "endpoints_fal-ai_cogvideox-5b_image-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_cogvideox-5b",
        feature = "endpoints_fal-ai_cogvideox-5b_image-to-video"
    )))
)]
pub mod image_to_video;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_cogvideox-5b",
    feature = "endpoints_fal-ai_cogvideox-5b_video-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_cogvideox-5b",
        feature = "endpoints_fal-ai_cogvideox-5b_video-to-video"
    )))
)]
pub mod video_to_video;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// The prompt used for generating the video.
    pub prompt: String,
    /// Seed of the generated video. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
    pub timings: Timings,
    /// The URL to the generated video
    pub video: File,
}

/// CogVideoX-5B
///
/// Category: text-to-video
/// Machine Type: H100
///
///
/// CogVideoX-5B, next generation text-to-video model.
pub fn cogvideox_5b(params: BaseInput) -> FalRequest<BaseInput, Output> {
    FalRequest::new("fal-ai/cogvideox-5b", params)
}
