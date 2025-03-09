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
    feature = "endpoints_fal-ai_pixverse_v3-5_text-to-video",
    feature = "endpoints_fal-ai_pixverse_v3-5_text-to-video_fast"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_pixverse",
        feature = "endpoints_fal-ai_pixverse_v3-5",
        feature = "endpoints_fal-ai_pixverse_v3-5_text-to-video",
        feature = "endpoints_fal-ai_pixverse_v3-5_text-to-video_fast"
    )))
)]
pub mod fast;

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoOutput {
    /// The generated video/// The generated video/// {"content_type":"video/mp4","file_name":"output.mp4","file_size":2995630,"url":"https://fal.media/files/zebra/11UahivZ3XZ1tRlcEcgPq_output.mp4"}
    pub video: File,
}

/// PixVerse v3.5
///
/// Category: text-to-video
/// Machine Type: A100
pub fn text_to_video(params: TextToVideoRequest) -> FalRequest<TextToVideoRequest, VideoOutput> {
    FalRequest::new("fal-ai/pixverse/v3.5/text-to-video", params)
}
