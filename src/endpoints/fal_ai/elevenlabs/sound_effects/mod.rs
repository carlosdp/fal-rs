#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SoundEffectOutput {
    /// The generated sound effect audio file in MP3 format/// The generated sound effect audio file in MP3 format/// {"url":"https://v3.fal.media/files/lion/WgnO-jy6WduosuG_Ibobx_sound_effect.mp3"}
    pub audio: File,
}

/// ElevenLabs Audio Isolation
///
/// Category: audio-to-audio
///
/// License Type: commercial
///
/// ElevenLabs Sound Effects Generation.
///
/// Turn text into sound effects for videos, voice-overs or video games using
/// state-of-the-art sound generation technology.
pub fn sound_effects(
    params: SoundEffectRequest,
) -> FalRequest<SoundEffectRequest, SoundEffectOutput> {
    FalRequest::new("fal-ai/elevenlabs/sound-effects", params)
}
