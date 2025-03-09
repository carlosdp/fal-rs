#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_ben",
    feature = "endpoints_fal-ai_ben_v2"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_ben",
        feature = "endpoints_fal-ai_ben_v2"
    )))
)]
pub mod v2;
