#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct I2VOutput {
    /// The generated video/// The generated video/// {"url":"https://v2.fal.media/files/8c216fcbc4ed41cd8840bd48c1ec6dd6_output.mp4"}
    pub video: File,
}

/// Luma Ray 2
///
/// Category: text-to-video
/// Machine Type: A100
pub fn image_to_video(params: ImageToVideoRequest) -> FalRequest<ImageToVideoRequest, I2VOutput> {
    FalRequest::new("fal-ai/luma-dream-machine/ray-2", params)
}
