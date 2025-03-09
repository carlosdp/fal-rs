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
    feature = "endpoints_fal-ai_minimax_image-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_minimax",
        feature = "endpoints_fal-ai_minimax_image-to-video"
    )))
)]
pub mod image_to_video;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_minimax",
    feature = "endpoints_fal-ai_minimax_video-01"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_minimax",
        feature = "endpoints_fal-ai_minimax_video-01"
    )))
)]
pub mod video_01;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_minimax",
    feature = "endpoints_fal-ai_minimax_video-01-director"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_minimax",
        feature = "endpoints_fal-ai_minimax_video-01-director"
    )))
)]
pub mod video_01_director;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_minimax",
    feature = "endpoints_fal-ai_minimax_video-01-live"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_minimax",
        feature = "endpoints_fal-ai_minimax_video-01-live"
    )))
)]
pub mod video_01_live;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_minimax",
    feature = "endpoints_fal-ai_minimax_video-01-subject-reference"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_minimax",
        feature = "endpoints_fal-ai_minimax_video-01-subject-reference"
    )))
)]
pub mod video_01_subject_reference;

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoOutput {
    /// The generated video/// The generated video/// {"url":"https://fal.media/files/monkey/vNZqQV_WgC9MhoidClLyw_output.mp4"}
    pub video: File,
}

/// MiniMax (Hailuo AI) Video 01 Live
///
/// Category: text-to-video
pub fn minimax(params: TextToVideoRequest) -> FalRequest<TextToVideoRequest, VideoOutput> {
    FalRequest::new("fal-ai/minimax", params)
}
