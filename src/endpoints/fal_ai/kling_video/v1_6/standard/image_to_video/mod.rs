#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct I2VOutput {
    /// The generated video
    /// {"url":"https://v2.fal.media/files/36087878b0c1435bb75c19b64b7db178_output.mp4"}
    pub video: File,
}

/// Kling 1.0
///
/// Category: text-to-video
/// Machine Type: A100
///
///
/// Kling 1.6 (std) Image to Video API.
pub fn image_to_video(params: ImageToVideoRequest) -> FalRequest<ImageToVideoRequest, I2VOutput> {
    FalRequest::new("fal-ai/kling-video/v1.6/standard/image-to-video", params)
}
