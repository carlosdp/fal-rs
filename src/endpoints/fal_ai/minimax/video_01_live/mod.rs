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
    feature = "endpoints_fal-ai_minimax_video-01-live",
    feature = "endpoints_fal-ai_minimax_video-01-live_image-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_minimax",
        feature = "endpoints_fal-ai_minimax_video-01-live",
        feature = "endpoints_fal-ai_minimax_video-01-live_image-to-video"
    )))
)]
pub mod image_to_video;

#[derive(Debug, Serialize, Deserialize)]
pub struct T2VLiveOutput {
    /// The generated video
    /// {"url":"https://fal.media/files/monkey/EbJRdZfaJbNiJBUvPta3c_output.mp4"}
    pub video: File,
}

/// MiniMax (Hailuo AI) Video 01 Live
///
/// Category: text-to-video
///
///
///
/// Hailuo T2V-01-Live text-to-video API: Transform Static Art into Dynamic Masterpieces
///
/// Live is our latest addition to the I2V family, designed to revolutionize how 2D illustrations come to life. With enhanced
/// smoothness and vivid motion, this model lets your characters move, speak, and shine like never before. Optimized for stability
/// and subtle expression, Hailuo I2V-01-Live supports a wide range of artistic styles, empowering you to expand your creative expression
/// and bring your art to life with unparalleled fluidity and finesse.
pub fn video_01_live(
    params: TextToVideoLiveRequest,
) -> FalRequest<TextToVideoLiveRequest, T2VLiveOutput> {
    FalRequest::new("fal-ai/minimax/video-01-live", params)
}
