#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

pub mod batch;
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
