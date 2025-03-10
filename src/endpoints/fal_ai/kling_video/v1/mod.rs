#[cfg(any(
    feature = "endpoints_fal-ai_kling-video_v1_pro",
    feature = "endpoints_fal-ai_kling-video_v1_pro_effects",
    feature = "endpoints_fal-ai_kling-video_v1_pro_image-to-video",
    feature = "endpoints_fal-ai_kling-video_v1_pro_text-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_kling-video_v1_pro",
        feature = "endpoints_fal-ai_kling-video_v1_pro_effects",
        feature = "endpoints_fal-ai_kling-video_v1_pro_image-to-video",
        feature = "endpoints_fal-ai_kling-video_v1_pro_text-to-video"
    )))
)]
pub mod pro;
#[cfg(any(
    feature = "endpoints_fal-ai_kling-video_v1_standard",
    feature = "endpoints_fal-ai_kling-video_v1_standard_effects",
    feature = "endpoints_fal-ai_kling-video_v1_standard_image-to-video",
    feature = "endpoints_fal-ai_kling-video_v1_standard_text-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_kling-video_v1_standard",
        feature = "endpoints_fal-ai_kling-video_v1_standard_effects",
        feature = "endpoints_fal-ai_kling-video_v1_standard_image-to-video",
        feature = "endpoints_fal-ai_kling-video_v1_standard_text-to-video"
    )))
)]
pub mod standard;
