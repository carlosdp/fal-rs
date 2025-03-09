#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// URL of the video's thumbnail image
    pub thumbnail_url: String,
    /// URL of the processed video file
    pub video_url: String,
}

/// FFmpeg API Compose
///
/// Category: video-to-video
/// Machine Type: L
pub fn compose(params: Input) -> FalRequest<Input, Output> {
    FalRequest::new("fal-ai/ffmpeg-api/compose", params)
}
