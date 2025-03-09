#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlNeXtOutput {
    /// The generated video.
    pub video: File,
}

/// ControlNeXt SVD
///
/// Category: video-to-video
/// Machine Type: A100
/// License Type: commercial
pub fn controlnext(params: ControlNeXtInput) -> FalRequest<ControlNeXtInput, ControlNeXtOutput> {
    FalRequest::new("fal-ai/controlnext", params)
}
