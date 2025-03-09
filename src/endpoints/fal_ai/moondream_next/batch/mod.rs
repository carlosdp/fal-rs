#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchMoonDreamOutput {
    /// URL to the generated captions JSON file containing filename-caption pairs.
    pub captions_file: File,
    /// List of generated captions
    pub outputs: Vec<String>,
}

/// MoonDreamNext
///
/// Category: vision
///
/// License Type: commercial
pub fn batch(params: BatchQueryInput) -> FalRequest<BatchQueryInput, BatchMoonDreamOutput> {
    FalRequest::new("fal-ai/moondream-next/batch", params)
}
