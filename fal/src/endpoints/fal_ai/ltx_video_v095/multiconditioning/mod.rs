#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MulticonditioningVideoOutput {
    /// The seed used for generation.
    pub seed: i64,
    /// The generated video file./// The generated video file./// {"url":"https://storage.googleapis.com/falserverless/gallery/ltx-multicondition.mp4"}
    pub video: File,
}

/// LTX Video-0.9.5
///
/// Category: text-to-video
/// Machine Type: H100
///
///
/// Generate a video from a prompt.
pub fn multiconditioning(
    params: MultiConditioningVideoInput,
) -> FalRequest<MultiConditioningVideoInput, MulticonditioningVideoOutput> {
    FalRequest::new("fal-ai/ltx-video-v095", params)
}
