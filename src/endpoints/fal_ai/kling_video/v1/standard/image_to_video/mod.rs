#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct KlingV1I2VOutput {
    /// The generated video/// The generated video/// {"url":"https://v3.fal.media/files/kangaroo/VUmAU3JvzS7mxwdgSU9zj_output.mp4"}
    pub video: File,
}

/// Kling 1.0
///
/// Category: text-to-video
/// Machine Type: A100
///
///
/// Kling 1.0 Image to Video API.
pub fn image_to_video(
    params: V1ImageToVideoRequest,
) -> FalRequest<V1ImageToVideoRequest, KlingV1I2VOutput> {
    FalRequest::new("fal-ai/kling-video/v1/standard/image-to-video", params)
}
