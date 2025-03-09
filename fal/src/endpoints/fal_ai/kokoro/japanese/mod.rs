#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct JapaneseOutput {
    /// The generated music/// The generated music/// {"url":"https://fal.media/files/lion/piLhqKO8LJxrWaNg2dVUv_tmpp6eff6zl.wav"}
    pub audio: File,
}

/// Kokoro TTS
///
/// Category: text-to-audio
/// Machine Type: A100
pub fn japanese(params: JapaneseRequest) -> FalRequest<JapaneseRequest, JapaneseOutput> {
    FalRequest::new("fal-ai/kokoro/japanese", params)
}
