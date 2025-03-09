#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kling",
    feature = "endpoints_fal-ai_kling_v1-5"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_kling",
        feature = "endpoints_fal-ai_kling_v1-5"
    )))
)]
pub mod v1_5;
