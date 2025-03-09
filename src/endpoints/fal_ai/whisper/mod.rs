#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct WhisperOutput {
    /// Timestamp chunks of the audio file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunks: Option<Vec<Option<WhisperChunk>>>,
    /// Speaker diarization segments of the audio file. Only present if diarization is enabled.
    pub diarization_segments: Vec<DiarizationSegment>,
    /// List of languages that the audio file is inferred to be. Defaults to null.
    pub inferred_languages: Vec<String>,
    /// Transcription of the audio file
    pub text: String,
}

/// Whisper
///
/// Category: speech-to-text
/// Machine Type: A100
pub fn whisper(params: WhisperInput) -> FalRequest<WhisperInput, WhisperOutput> {
    FalRequest::new("fal-ai/whisper", params)
}
