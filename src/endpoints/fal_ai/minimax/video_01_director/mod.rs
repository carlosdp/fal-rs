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
    feature = "endpoints_fal-ai_minimax_video-01-director",
    feature = "endpoints_fal-ai_minimax_video-01-director_image-to-video"
))]
pub mod image_to_video;

#[derive(Debug, Serialize, Deserialize)]
pub struct T2VDirectorOutput {
    /// The generated video/// The generated video/// {"url":"https://fal.media/files/panda/4Et1qL4cbedh-OACEw7OF_output.mp4"}
    pub video: File,
}

/// MiniMax (Hailuo AI) Video 01 Live
///
/// Category: text-to-video
///
///
///
/// Hailuo T2V-01-Director API: Text-to-video generation with precise camera control for cinematic storytelling
pub fn video_01_director(
    params: TextToVideoDirectorRequest,
) -> FalRequest<TextToVideoDirectorRequest, T2VDirectorOutput> {
    FalRequest::new("fal-ai/minimax/video-01-director", params)
}
