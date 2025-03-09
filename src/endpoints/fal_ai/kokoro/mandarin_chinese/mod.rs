#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MandarinOutput {
    /// The generated music
    /// {"url":"https://fal.media/files/rabbit/8UiqobkQXPrYDRHl4l5oU_tmptz6jo3ex.wav"}
    pub audio: File,
}

/// Kokoro TTS
///
/// Category: text-to-audio
/// Machine Type: A100
pub fn mandarin_chinese(params: MandarinRequest) -> FalRequest<MandarinRequest, MandarinOutput> {
    FalRequest::new("fal-ai/kokoro/mandarin-chinese", params)
}
