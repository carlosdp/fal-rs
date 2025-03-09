#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageToVideoOutput {
    /// The seed used for generation.
    pub seed: i64,
    /// The generated video file./// The generated video file./// {"url":"https://storage.googleapis.com/falserverless/example_outputs/ltx_i2v_output.mp4"}
    pub video: File,
}

/// LTX Video-0.9.5
///
/// Category: text-to-video
/// Machine Type: H100
///
///
/// Generate a video from a prompt.
pub fn image_to_video(
    params: ImageToVideoInput,
) -> FalRequest<ImageToVideoInput, ImageToVideoOutput> {
    FalRequest::new("fal-ai/ltx-video-v095/image-to-video", params)
}
