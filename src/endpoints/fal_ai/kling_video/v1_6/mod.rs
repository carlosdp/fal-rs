#[cfg(any(
    feature = "endpoints_fal-ai_kling-video_v1-6_pro",
    feature = "endpoints_fal-ai_kling-video_v1-6_pro_effects",
    feature = "endpoints_fal-ai_kling-video_v1-6_pro_image-to-video",
    feature = "endpoints_fal-ai_kling-video_v1-6_pro_text-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_kling-video_v1-6_pro",
        feature = "endpoints_fal-ai_kling-video_v1-6_pro_effects",
        feature = "endpoints_fal-ai_kling-video_v1-6_pro_image-to-video",
        feature = "endpoints_fal-ai_kling-video_v1-6_pro_text-to-video"
    )))
)]
pub mod pro;
#[cfg(any(
    feature = "endpoints_fal-ai_kling-video_v1-6_standard",
    feature = "endpoints_fal-ai_kling-video_v1-6_standard_effects",
    feature = "endpoints_fal-ai_kling-video_v1-6_standard_image-to-video",
    feature = "endpoints_fal-ai_kling-video_v1-6_standard_text-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_kling-video_v1-6_standard",
        feature = "endpoints_fal-ai_kling-video_v1-6_standard_effects",
        feature = "endpoints_fal-ai_kling-video_v1-6_standard_image-to-video",
        feature = "endpoints_fal-ai_kling-video_v1-6_standard_text-to-video"
    )))
)]
pub mod standard;
