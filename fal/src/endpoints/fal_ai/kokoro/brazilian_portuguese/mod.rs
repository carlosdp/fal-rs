#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct BrPortugeseOutput {
    /// The generated music/// The generated music/// {"url":"https://fal.media/files/rabbit/Y9-bWJt5lixo8PTCmncN6_tmpyh7u57oa.wav"}
    pub audio: File,
}

/// Kokoro TTS
///
/// Category: text-to-audio
/// Machine Type: A100
pub fn brazilian_portuguese(
    params: BrPortugueseRequest,
) -> FalRequest<BrPortugueseRequest, BrPortugeseOutput> {
    FalRequest::new("fal-ai/kokoro/brazilian-portuguese", params)
}
