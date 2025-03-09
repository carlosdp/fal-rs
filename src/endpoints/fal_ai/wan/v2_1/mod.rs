#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_wan",
    feature = "endpoints_fal-ai_wan_v2-1",
    feature = "endpoints_fal-ai_wan_v2-1_v1-3b"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_wan",
        feature = "endpoints_fal-ai_wan_v2-1",
        feature = "endpoints_fal-ai_wan_v2-1_v1-3b"
    )))
)]
pub mod v1_3b;
