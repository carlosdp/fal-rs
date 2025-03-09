#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_minimax",
    feature = "endpoints_fal-ai_minimax_video-01",
    feature = "endpoints_fal-ai_minimax_video-01_image-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_minimax",
        feature = "endpoints_fal-ai_minimax_video-01",
        feature = "endpoints_fal-ai_minimax_video-01_image-to-video"
    )))
)]
pub mod image_to_video;

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoOutput {
    /// The generated video/// The generated video/// {"url":"https://fal.media/files/monkey/vNZqQV_WgC9MhoidClLyw_output.mp4"}
    pub video: File,
}

/// MiniMax (Hailuo AI) Video 01 Live
///
/// Category: text-to-video
///
///
///
/// Hailuo T2V-01 API: Native high-resolution, high-frame-rate video generation model, supports text-to-video and image-to-video
pub fn video_01(params: TextToVideoRequest) -> FalRequest<TextToVideoRequest, VideoOutput> {
    FalRequest::new("fal-ai/minimax/video-01", params)
}
