#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

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
    FalRequest::new("fal-ai/minimax/video-01-live", params)
}
