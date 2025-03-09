#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_playai",
    feature = "endpoints_fal-ai_playai_create-voice"
))]
pub mod create_voice;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_playai",
    feature = "endpoints_fal-ai_playai_train"
))]
pub mod train;
