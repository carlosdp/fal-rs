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
/// ElevenLabs Turbo v2.5 model for text-to-speech generation.
///
/// High quality with lowest latency, ideal for real-time applications.
/// Supports 32 languages while maintaining natural voice quality.
pub fn turbo_v2_5(params: TextToSpeechRequest) -> FalRequest<TextToSpeechRequest, TTSOutput> {
    FalRequest::new("fal-ai/elevenlabs/audio-isolation", params)
}
