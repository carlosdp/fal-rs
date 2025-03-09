#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// The prompt used for generating the video.
    pub prompt: String,
    /// Seed of the generated video. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
    pub timings: Timings,
    /// The URL to the generated video
    pub video: File,
}

/// CogVideoX-5B
///
/// Category: text-to-video
/// Machine Type: H100
///
///
/// Video to video generation using CogVideoX-5B.
pub fn video_to_video(params: VideoToVideoInput) -> FalRequest<VideoToVideoInput, Output> {
    FalRequest::new("fal-ai/cogvideox-5b", params)
}
