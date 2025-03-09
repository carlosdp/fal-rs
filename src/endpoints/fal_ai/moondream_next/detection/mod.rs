#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectionOutput {
    /// Output image with detection visualization
    pub image: Image,
    /// Detection results as text
    pub text_output: String,
}

/// MoonDreamNext
///
/// Category: vision
///
/// License Type: commercial
pub fn detection(params: DetectionInput) -> FalRequest<DetectionInput, DetectionOutput> {
    FalRequest::new("fal-ai/moondream-next/detection", params)
}
