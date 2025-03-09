#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

pub mod video;

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
pub fn mini_cpm(
    params: MiniCPMV26ImageInput,
) -> FalRequest<MiniCPMV26ImageInput, MiniCPMV26Output> {
    FalRequest::new("fal-ai/mini-cpm", params)
}
