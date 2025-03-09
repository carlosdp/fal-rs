#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpanishOutput {
    /// The generated music
    /// {"url":"https://fal.media/files/monkey/5rBM3qVCED73Lxs5XLcwj_tmp4f2z_qrf.wav"}
    pub audio: File,
}

/// Kokoro TTS
///
/// Category: text-to-audio
/// Machine Type: A100
pub fn spanish(params: SpanishRequest) -> FalRequest<SpanishRequest, SpanishOutput> {
    FalRequest::new("fal-ai/kokoro/spanish", params)
}
