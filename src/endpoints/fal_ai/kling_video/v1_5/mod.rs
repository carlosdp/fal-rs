#[cfg(any(
    feature = "endpoints_fal-ai_kling-video_v1-5_pro",
    feature = "endpoints_fal-ai_kling-video_v1-5_pro_effects",
    feature = "endpoints_fal-ai_kling-video_v1-5_pro_image-to-video",
    feature = "endpoints_fal-ai_kling-video_v1-5_pro_text-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_kling-video_v1-5_pro",
        feature = "endpoints_fal-ai_kling-video_v1-5_pro_effects",
        feature = "endpoints_fal-ai_kling-video_v1-5_pro_image-to-video",
        feature = "endpoints_fal-ai_kling-video_v1-5_pro_text-to-video"
    )))
)]
pub mod pro;
#[cfg(any(
    feature = "endpoints_fal-ai_kling-video_v1-5_standard",
    feature = "endpoints_fal-ai_kling-video_v1-5_standard_effects"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_kling-video_v1-5_standard",
        feature = "endpoints_fal-ai_kling-video_v1-5_standard_effects"
    )))
)]
pub mod standard;
