#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct BrEngOutput {
    /// The generated music/// The generated music/// {"url":"https://fal.media/files/kangaroo/4wpA60Kum6UjOVBKJoNyL_tmpxfrkn95k.wav"}
    pub audio: File,
}

/// Kokoro TTS
///
/// Category: text-to-audio
/// Machine Type: A100
pub fn british_english(params: BrEnglishRequest) -> FalRequest<BrEnglishRequest, BrEngOutput> {
    FalRequest::new("fal-ai/kokoro/american-english", params)
}
