#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct HunyuanT2VResponse {
    /// The seed used for generating the video.
    pub seed: i64,
    pub video: File,
}

/// Hunyuan Video
///
/// Category: text-to-video
/// Machine Type: H100
///
///
/// Hunyuan Video API for fast video generation. Text-to-video and video-to-video modes are supported.
pub fn video_to_video(
    params: HunyuanV2VRequest,
) -> FalRequest<HunyuanV2VRequest, HunyuanT2VResponse> {
    FalRequest::new("fal-ai/hunyuan-video/video-to-video", params)
}
