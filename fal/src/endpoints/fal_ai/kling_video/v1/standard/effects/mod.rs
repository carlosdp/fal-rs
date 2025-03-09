#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoEffectsOutput {
    /// The generated video/// The generated video/// {"content_type":"video/mp4","file_name":"output.mp4","url":"https://storage.googleapis.com/falserverless/kling/kling_ex.mp4.mp4"}
    pub video: File,
}

/// Kling 1.0
///
/// Category: text-to-video
/// Machine Type: A100
///
///
/// Kling 1.0 (std) Effects API.
pub fn effects(params: VideoEffectsRequest) -> FalRequest<VideoEffectsRequest, VideoEffectsOutput> {
    FalRequest::new("fal-ai/kling-video/v1/standard/text-to-video", params)
}
