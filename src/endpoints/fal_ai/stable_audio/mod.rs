#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// The generated audio clip
    pub audio_file: File,
}

/// Stable Audio Open
///
/// Category: text-to-audio
/// Machine Type: A100
pub fn stable_audio(params: Input) -> FalRequest<Input, Output> {
    FalRequest::new("fal-ai/stable-audio", params)
}
