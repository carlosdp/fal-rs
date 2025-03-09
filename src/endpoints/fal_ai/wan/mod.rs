#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_wan",
    feature = "endpoints_fal-ai_wan_v2-1"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_wan",
        feature = "endpoints_fal-ai_wan_v2-1"
    )))
)]
pub mod v2_1;
