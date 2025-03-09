#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

pub mod image_to_video;
pub mod video_01;
pub mod video_01_director;
pub mod video_01_live;
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
