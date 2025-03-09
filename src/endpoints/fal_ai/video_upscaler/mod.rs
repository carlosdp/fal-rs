#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// The stitched video
    /// {"content_type":"video/mp4","url":"https://storage.googleapis.com/falserverless/videos/h0jgPaO6AJAbyrsNYNbGl_upscaled_video.mp4"}
    pub video: File,
}

/// Video Upscaler
///
/// Category: video-to-video
/// Machine Type: A6000
pub fn video_upscaler(params: Input) -> FalRequest<Input, Output> {
    FalRequest::new("fal-ai/video-upscaler", params)
}
