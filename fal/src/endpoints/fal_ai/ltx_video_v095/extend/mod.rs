#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendVideoOutput {
    /// The seed used for generation.
    pub seed: i64,
    /// The generated video file./// The generated video file./// {"url":"https://storage.googleapis.com/falserverless/example_outputs/ltx-v095_extend.mp4"}
    pub video: File,
}

/// LTX Video-0.9.5
///
/// Category: text-to-video
/// Machine Type: H100
///
///
/// Generate a video from a prompt.
pub fn extend(params: ExtendVideoInput) -> FalRequest<ExtendVideoInput, ExtendVideoOutput> {
    FalRequest::new("fal-ai/ltx-video-v095", params)
}
