#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoOutput {
    /// The generated video
    /// {"content_type":"video/mp4","file_name":"output.mp4","file_size":2995630,"url":"https://fal.media/files/zebra/11UahivZ3XZ1tRlcEcgPq_output.mp4"}
    pub video: File,
}

/// PixVerse v3.5
///
/// Category: text-to-video
/// Machine Type: A100
pub fn fast(params: FastTextToVideoRequest) -> FalRequest<FastTextToVideoRequest, VideoOutput> {
    FalRequest::new("fal-ai/pixverse/v3.5/text-to-video/fast", params)
}
