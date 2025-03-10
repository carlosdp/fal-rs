#[cfg(any(feature = "endpoints_fal-ai_elevenlabs_audio-isolation"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_elevenlabs_audio-isolation")))
)]
pub mod audio_isolation;
#[cfg(any(feature = "endpoints_fal-ai_elevenlabs_sound-effects"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_elevenlabs_sound-effects")))
)]
pub mod sound_effects;
#[cfg(any(feature = "endpoints_fal-ai_elevenlabs_speech-to-text"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_elevenlabs_speech-to-text")))
)]
pub mod speech_to_text;
#[cfg(any(
    feature = "endpoints_fal-ai_elevenlabs_tts",
    feature = "endpoints_fal-ai_elevenlabs_tts_multilingual-v2",
    feature = "endpoints_fal-ai_elevenlabs_tts_turbo-v2-5"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_elevenlabs_tts",
        feature = "endpoints_fal-ai_elevenlabs_tts_multilingual-v2",
        feature = "endpoints_fal-ai_elevenlabs_tts_turbo-v2-5"
    )))
)]
pub mod tts;
