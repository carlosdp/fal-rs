#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kling-video",
    feature = "endpoints_fal-ai_kling-video_v1-5",
    feature = "endpoints_fal-ai_kling-video_v1-5_pro",
    feature = "endpoints_fal-ai_kling-video_v1-5_pro_effects"
))]
pub mod effects;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kling-video",
    feature = "endpoints_fal-ai_kling-video_v1-5",
    feature = "endpoints_fal-ai_kling-video_v1-5_pro",
    feature = "endpoints_fal-ai_kling-video_v1-5_pro_image-to-video"
))]
pub mod image_to_video;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kling-video",
    feature = "endpoints_fal-ai_kling-video_v1-5",
    feature = "endpoints_fal-ai_kling-video_v1-5_pro",
    feature = "endpoints_fal-ai_kling-video_v1-5_pro_text-to-video"
))]
pub mod text_to_video;
