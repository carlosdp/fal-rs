#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MiniCPMV26Output {
    /// Response from the model
    pub output: String,
}

/// MiniCPM-V 2.6
///
/// Category: vision
/// Machine Type: A100
/// License Type: commercial
pub fn video(params: MiniCPMV26VideoInput) -> FalRequest<MiniCPMV26VideoInput, MiniCPMV26Output> {
    FalRequest::new("fal-ai/mini-cpm/video", params)
}
