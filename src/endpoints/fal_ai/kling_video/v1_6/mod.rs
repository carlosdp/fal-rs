#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kling-video",
    feature = "endpoints_fal-ai_kling-video_v1-6",
    feature = "endpoints_fal-ai_kling-video_v1-6_pro"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_kling-video",
        feature = "endpoints_fal-ai_kling-video_v1-6",
        feature = "endpoints_fal-ai_kling-video_v1-6_pro"
    )))
)]
pub mod pro;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kling-video",
    feature = "endpoints_fal-ai_kling-video_v1-6",
    feature = "endpoints_fal-ai_kling-video_v1-6_standard"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_kling-video",
        feature = "endpoints_fal-ai_kling-video_v1-6",
        feature = "endpoints_fal-ai_kling-video_v1-6_standard"
    )))
)]
pub mod standard;
