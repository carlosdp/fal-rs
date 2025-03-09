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
    /// The URL to the generated video/// The URL to the generated video/// [{"content_type":"application/octet-stream","file_name":"rgb.mp4","file_size":146468,"url":"https://v3.fal.media/files/kangaroo/G6gkFsuyU5L7sJ55nZUPU_rgb.mp4"},{"content_type":"application/octet-stream","file_name":"alpha.mp4","file_size":106894,"url":"https://v3.fal.media/files/lion/g7PBZfQEH9SoPXYgeyl5P_alpha.mp4"}]
    pub videos: Vec<File>,
}

/// TransPixar
///
/// Category: text-to-video
/// Machine Type: H100
///
///
/// TransPixar: Backgroundless video generation model
pub fn transpixar(params: BaseInput) -> FalRequest<BaseInput, Output> {
    FalRequest::new("fal-ai/transpixar", params)
}
