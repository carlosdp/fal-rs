#[cfg(any(feature = "endpoints_fal-ai_kling-video_v1_pro_effects"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_kling-video_v1_pro_effects")))
)]
pub mod effects;
#[cfg(any(feature = "endpoints_fal-ai_kling-video_v1_pro_image-to-video"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_kling-video_v1_pro_image-to-video")))
)]
pub mod image_to_video;
#[cfg(any(feature = "endpoints_fal-ai_kling-video_v1_pro_text-to-video"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_kling-video_v1_pro_text-to-video")))
)]
pub mod text_to_video;
