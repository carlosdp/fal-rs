#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_pixverse",
    feature = "endpoints_fal-ai_pixverse_v3-5",
    feature = "endpoints_fal-ai_pixverse_v3-5_image-to-video",
    feature = "endpoints_fal-ai_pixverse_v3-5_image-to-video_fast"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_pixverse",
        feature = "endpoints_fal-ai_pixverse_v3-5",
        feature = "endpoints_fal-ai_pixverse_v3-5_image-to-video",
        feature = "endpoints_fal-ai_pixverse_v3-5_image-to-video_fast"
    )))
)]
pub mod fast;

#[derive(Debug, Serialize, Deserialize)]
pub struct I2VOutput {
    /// The generated video/// The generated video/// {"content_type":"video/mp4","file_name":"output.mp4","file_size":4060052,"url":"https://fal.media/files/tiger/8V9H8RLyFiWjmJDOxGbcG_output.mp4"}
    pub video: File,
}

/// PixVerse v3.5
///
/// Category: text-to-video
/// Machine Type: A100
pub fn image_to_video(params: ImageToVideoRequest) -> FalRequest<ImageToVideoRequest, I2VOutput> {
    FalRequest::new("fal-ai/pixverse/v3.5/image-to-video", params)
}
