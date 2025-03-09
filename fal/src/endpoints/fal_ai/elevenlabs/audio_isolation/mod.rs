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
/// ElevenLabs Audio Isolation: Extract clean voice from audio.
///
/// Isolates and enhances voice content from audio files, removing background noise,
/// music, and other non-voice sounds.
pub fn audio_isolation(
    params: AudioIsolationRequest,
) -> FalRequest<AudioIsolationRequest, TTSOutput> {
    FalRequest::new("fal-ai/elevenlabs/audio-isolation", params)
}
