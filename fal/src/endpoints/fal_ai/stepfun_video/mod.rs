#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct StepFunT2VResponse {
    /// The seed used for generating the video.
    pub seed: i64,
    pub video: File,
}

/// Step-Video
///
/// Category: text-to-video
/// Machine Type: H100
pub fn stepfun_video(
    params: StepFunT2VRequest,
) -> FalRequest<StepFunT2VRequest, StepFunT2VResponse> {
    FalRequest::new("fal-ai/stepfun-video", params)
}
