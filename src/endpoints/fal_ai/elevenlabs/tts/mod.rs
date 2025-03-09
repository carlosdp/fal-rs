#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_elevenlabs",
    feature = "endpoints_fal-ai_elevenlabs_tts",
    feature = "endpoints_fal-ai_elevenlabs_tts_multilingual-v2"
))]
pub mod multilingual_v2;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_elevenlabs",
    feature = "endpoints_fal-ai_elevenlabs_tts",
    feature = "endpoints_fal-ai_elevenlabs_tts_turbo-v2-5"
))]
pub mod turbo_v2_5;
