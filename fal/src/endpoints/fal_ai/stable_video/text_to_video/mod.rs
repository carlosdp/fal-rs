#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoOutput {
    /// Seed for random number generator
    pub seed: i64,
    /// Generated video
    pub video: File,
}

/// High Quality Stable Video Diffusion
///
/// Category: image-to-video
/// Machine Type: A100
pub fn text_to_video(params: TextInput) -> FalRequest<TextInput, VideoOutput> {
    FalRequest::new("fal-ai/stable-video/text-to-video", params)
}
