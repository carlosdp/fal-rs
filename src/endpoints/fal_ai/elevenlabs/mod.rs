#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_elevenlabs",
    feature = "endpoints_fal-ai_elevenlabs_audio-isolation"
))]
pub mod audio_isolation;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_elevenlabs",
    feature = "endpoints_fal-ai_elevenlabs_sound-effects"
))]
pub mod sound_effects;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_elevenlabs",
    feature = "endpoints_fal-ai_elevenlabs_speech-to-text"
))]
pub mod speech_to_text;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_elevenlabs",
    feature = "endpoints_fal-ai_elevenlabs_tts"
))]
pub mod tts;
