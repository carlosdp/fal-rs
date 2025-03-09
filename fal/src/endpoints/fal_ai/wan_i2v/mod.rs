#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct WanI2VResponse {
    /// The seed used for generation.
    pub seed: i64,
    /// The generated video file./// The generated video file./// {"url":"https://v3.fal.media/files/elephant/Nj4jZupkZvR7g0QkNueJZ_video-1740522225.mp4"}
    pub video: File,
}

/// Wan-2.1 Image-to-Video
///
/// Category: image-to-video
/// Machine Type: H100
///
///
/// Generate a video from an image and prompt.
pub fn wan_i2v(params: WanI2VRequest) -> FalRequest<WanI2VRequest, WanI2VResponse> {
    FalRequest::new("fal-ai/wan-i2v", params)
}
