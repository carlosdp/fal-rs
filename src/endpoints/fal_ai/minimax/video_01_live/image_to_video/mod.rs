#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct I2VLiveOutput {
    /// The generated video
    /// {"url":"https://fal.media/files/monkey/bkT4T4uLOXr0jDsIMlNd5_output.mp4"}
    pub video: File,
}

/// MiniMax (Hailuo AI) Video 01 Live
///
/// Category: text-to-video
///
///
///
/// Hailuo I2V-01-Live image-to-video API: Transform Static Art into Dynamic Masterpieces
///
/// Live is our latest addition to the I2V family, designed to revolutionize how 2D illustrations come to life. With
/// enhanced smoothness and vivid motion, this model lets your characters move, speak, and shine like never before. Optimized
/// for stability and subtle expression, Hailuo I2V-01-Live supports a wide range of artistic styles, empowering you to expand
/// your creative expression and bring your art to life with unparalleled fluidity and finesse.
pub fn image_to_video(
    params: ImageToVideoRequest,
) -> FalRequest<ImageToVideoRequest, I2VLiveOutput> {
    FalRequest::new("fal-ai/minimax/video-01-live/image-to-video", params)
}
