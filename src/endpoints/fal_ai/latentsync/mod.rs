#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// The generated video with the lip sync.
    pub video: File,
}

/// LatentSync
///
/// Category: video-to-video
///
/// License Type: commercial
pub fn latentsync(params: Input) -> FalRequest<Input, Output> {
    FalRequest::new("fal-ai/latentsync", params)
}
