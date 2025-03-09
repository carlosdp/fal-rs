#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchedMoondreamOutput {
    /// Filenames of the images processed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filenames: Option<Vec<Option<String>>>,
    /// List of generated outputs
    pub outputs: Vec<String>,
    /// Whether the output is partial
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<bool>,
    /// Timings for different parts of the process
    pub timings: Timings,
}

/// Moondream
///
/// Category: vision
/// Machine Type: A6000
pub fn batched(
    params: BatchedMoondreamInput,
) -> FalRequest<BatchedMoondreamInput, BatchedMoondreamOutput> {
    FalRequest::new("fal-ai/moondream/batched", params)
}
