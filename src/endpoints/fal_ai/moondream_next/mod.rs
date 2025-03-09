#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_moondream-next",
    feature = "endpoints_fal-ai_moondream-next_batch"
))]
pub mod batch;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_moondream-next",
    feature = "endpoints_fal-ai_moondream-next_detection"
))]
pub mod detection;

#[derive(Debug, Serialize, Deserialize)]
pub struct MoonDreamOutput {
    /// Response from the model
    pub output: String,
}

/// MoonDreamNext
///
/// Category: vision
///
/// License Type: commercial
pub fn moondream_next(params: QueryInput) -> FalRequest<QueryInput, MoonDreamOutput> {
    FalRequest::new("fal-ai/moondream-next", params)
}
