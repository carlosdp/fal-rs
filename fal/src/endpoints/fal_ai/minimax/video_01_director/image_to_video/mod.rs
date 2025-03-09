#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct I2VDirectorOutput {
    /// The generated video/// The generated video/// {"url":"https://storage.googleapis.com/falserverless/web-examples/minimax/i2v-01.mp4"}
    pub video: File,
}

/// MiniMax (Hailuo AI) Video 01 Live
///
/// Category: text-to-video
///
///
///
/// Hailuo I2V-01-Director API: Image-to-video generation with precise camera control for cinematic storytelling
pub fn image_to_video(
    params: ImageToVideoDirectorRequest,
) -> FalRequest<ImageToVideoDirectorRequest, I2VDirectorOutput> {
    FalRequest::new("fal-ai/minimax/video-01-director/image-to-video", params)
}
