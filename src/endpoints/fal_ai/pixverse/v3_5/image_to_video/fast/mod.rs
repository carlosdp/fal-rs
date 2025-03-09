#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct I2VOutput {
    /// The generated video
    /// {"content_type":"video/mp4","file_name":"output.mp4","file_size":4060052,"url":"https://fal.media/files/tiger/8V9H8RLyFiWjmJDOxGbcG_output.mp4"}
    pub video: File,
}

/// PixVerse v3.5
///
/// Category: text-to-video
/// Machine Type: A100
pub fn fast(params: FastImageToVideoRequest) -> FalRequest<FastImageToVideoRequest, I2VOutput> {
    FalRequest::new("fal-ai/pixverse/v3.5/image-to-video/fast", params)
}
