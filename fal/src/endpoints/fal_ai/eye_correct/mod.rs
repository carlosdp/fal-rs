#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct EyeCorrectOutput {
    /// The corrected video
    pub video: File,
}

/// Eye Correct
///
/// Category: video-to-video
/// Machine Type: A100
pub fn eye_correct(params: EyeCorrectInput) -> FalRequest<EyeCorrectInput, EyeCorrectOutput> {
    FalRequest::new("fal-ai/eye-correct", params)
}
