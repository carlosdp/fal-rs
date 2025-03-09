#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_moondream",
    feature = "endpoints_fal-ai_moondream_batched"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_moondream",
        feature = "endpoints_fal-ai_moondream_batched"
    )))
)]
pub mod batched;
