#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TTSOutput {
    /// The generated audio file/// The generated audio file/// {"url":"https://v3.fal.media/files/zebra/zJL_oRY8h5RWwjoK1w7tx_output.mp3"}
    pub audio: File,
}

/// ElevenLabs Audio Isolation
///
/// Category: audio-to-audio
///
/// License Type: commercial
///
/// ElevenLabs Multilingual v2 model for text-to-speech generation.
///
/// Excels in stability, language diversity, and accent accuracy.
/// Supports 29 languages with high-quality, natural-sounding voices.
pub fn multilingual_v2(params: TextToSpeechRequest) -> FalRequest<TextToSpeechRequest, TTSOutput> {
    FalRequest::new("fal-ai/elevenlabs/audio-isolation", params)
}
