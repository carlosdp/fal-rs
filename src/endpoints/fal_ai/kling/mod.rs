#[cfg(any(
    feature = "endpoints_fal-ai_kling_v1-5",
    feature = "endpoints_fal-ai_kling_v1-5_kolors-virtual-try-on"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_kling_v1-5",
        feature = "endpoints_fal-ai_kling_v1-5_kolors-virtual-try-on"
    )))
)]
pub mod v1_5;
