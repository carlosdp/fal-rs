#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoOutput {
    /// The generated video
    /// {"url":"https://fal.media/files/monkey/vNZqQV_WgC9MhoidClLyw_output.mp4"}
    pub video: File,
}

/// MiniMax (Hailuo AI) Video 01 Live
///
/// Category: text-to-video
pub fn image_to_video(params: ImageToVideoRequest) -> FalRequest<ImageToVideoRequest, VideoOutput> {
    FalRequest::new("fal-ai/minimax/image-to-video", params)
}
