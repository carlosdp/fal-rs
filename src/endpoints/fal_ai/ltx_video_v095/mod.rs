#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_ltx-video-v095",
    feature = "endpoints_fal-ai_ltx-video-v095_extend"
))]
pub mod extend;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_ltx-video-v095",
    feature = "endpoints_fal-ai_ltx-video-v095_image-to-video"
))]
pub mod image_to_video;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_ltx-video-v095",
    feature = "endpoints_fal-ai_ltx-video-v095_multiconditioning"
))]
pub mod multiconditioning;

#[derive(Debug, Serialize, Deserialize)]
pub struct TextToVideoOutput {
    /// The seed used for generation.
    pub seed: i64,
    /// The generated video file./// The generated video file./// {"url":"https://storage.googleapis.com/falserverless/example_outputs/ltx-t2v_output.mp4"}
    pub video: File,
}

/// LTX Video-0.9.5
///
/// Category: text-to-video
/// Machine Type: H100
///
///
/// Generate a video from a prompt.
pub fn ltx_video_v095(params: TextToVideoInput) -> FalRequest<TextToVideoInput, TextToVideoOutput> {
    FalRequest::new("fal-ai/ltx-video-v095", params)
}
