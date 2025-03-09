#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_mmaudio-v2",
    feature = "endpoints_fal-ai_mmaudio-v2_text-to-audio"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_mmaudio-v2",
        feature = "endpoints_fal-ai_mmaudio-v2_text-to-audio"
    )))
)]
pub mod text_to_audio;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// The generated video with the lip sync.
    /// {"content_type":"application/octet-stream","file_name":"mmaudio_input.mp4","file_size":1001342,"url":"https://storage.googleapis.com/falserverless/model_tests/video_models/mmaudio_output.mp4"}
    pub video: File,
}

/// MMAudio V2
///
/// Category: video-to-video
/// Machine Type: A100
pub fn mmaudio_v2(params: BaseInput) -> FalRequest<BaseInput, Output> {
    FalRequest::new("fal-ai/mmaudio-v2", params)
}
