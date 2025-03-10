#[cfg(any(
    feature = "endpoints_fal-ai_wan_v2-1",
    feature = "endpoints_fal-ai_wan_v2-1_v1-3b",
    feature = "endpoints_fal-ai_wan_v2-1_v1-3b_text-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_wan_v2-1",
        feature = "endpoints_fal-ai_wan_v2-1_v1-3b",
        feature = "endpoints_fal-ai_wan_v2-1_v1-3b_text-to-video"
    )))
)]
pub mod v2_1;
