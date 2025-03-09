#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct WhisperOutput {
    /// Timestamp chunks of the audio file
    pub chunks: Vec<WhisperChunk>,
    /// Transcription of the audio file
    pub text: String,
}

/// Wizper (Whisper v3 -- fal.ai edition)
///
/// Category: speech-to-text
/// Machine Type: A100
///
///
/// Transcribe an audio file using the Whisper model.
pub fn wizper(params: WhisperInput) -> FalRequest<WhisperInput, WhisperOutput> {
    FalRequest::new("fal-ai/wizper", params)
}
