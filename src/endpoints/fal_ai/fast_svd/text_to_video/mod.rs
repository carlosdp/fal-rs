#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct FastSVDOutput {
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
    /// The generated video file.
    pub video: File,
}

/// Stable Video Diffusion
///
/// Category: text-to-video
/// Machine Type: A100
pub fn text_to_video(params: FastSVDTextInput) -> FalRequest<FastSVDTextInput, FastSVDOutput> {
    FalRequest::new("fal-ai/fast-svd/text-to-video", params)
}
