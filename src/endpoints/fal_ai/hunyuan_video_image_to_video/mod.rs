#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct HunyuanI2VResponse {
    /// The seed used for generating the video.
    pub seed: i64,
    pub video: File,
}

/// Hunyuan Video Image-to-Video Inference
///
/// Category: image-to-video
/// Machine Type: H100
pub fn hunyuan_video_image_to_video(
    params: HunyuanVideoRequest,
) -> FalRequest<HunyuanVideoRequest, HunyuanI2VResponse> {
    FalRequest::new("fal-ai/hunyuan-video-image-to-video", params)
}
