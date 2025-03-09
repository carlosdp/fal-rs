#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

pub mod video_to_video;

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
pub fn hunyuan_video(
    params: HunyuanVideoRequest,
) -> FalRequest<HunyuanVideoRequest, HunyuanT2VResponse> {
    FalRequest::new("fal-ai/hunyuan-video", params)
}
