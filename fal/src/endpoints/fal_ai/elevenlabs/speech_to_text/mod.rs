#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TranscriptionOutput {
    /// Detected or specified language code
    pub language_code: String,
    /// Confidence in language detection
    pub language_probability: f64,
    /// The full transcribed text
    pub text: String,
    /// Word-level transcription details
    pub words: Vec<TranscriptionWord>,
}

/// ElevenLabs Audio Isolation
///
/// Category: audio-to-audio
///
/// License Type: commercial
///
/// ElevenLabs Speech to Text: Transcribe audio with high accuracy.
///
/// Transforms spoken audio into text with word-level timestamps and
/// optional speaker identification. Supports 99 languages with state-of-the-art accuracy.
pub fn speech_to_text(
    params: SpeechToTextRequest,
) -> FalRequest<SpeechToTextRequest, TranscriptionOutput> {
    FalRequest::new("fal-ai/elevenlabs/audio-isolation", params)
}
