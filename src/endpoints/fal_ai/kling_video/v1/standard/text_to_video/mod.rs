#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct T2VOutput {
    /// The generated video
    /// {"url":"https://v2.fal.media/files/fb33a862b94d4d7195e610e4cbc5d392_output.mp4"}
    pub video: File,
}

/// Kling 1.0
///
/// Category: text-to-video
/// Machine Type: A100
///
///
/// Kling 1.0 Text to Video API.
pub fn text_to_video(params: V1TextToVideoRequest) -> FalRequest<V1TextToVideoRequest, T2VOutput> {
    FalRequest::new("fal-ai/kling-video/v1/standard/text-to-video", params)
}
