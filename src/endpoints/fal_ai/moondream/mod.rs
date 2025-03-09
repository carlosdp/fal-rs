#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_moondream",
    feature = "endpoints_fal-ai_moondream_batched"
))]
pub mod batched;
