#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kling-video",
    feature = "endpoints_fal-ai_kling-video_v1",
    feature = "endpoints_fal-ai_kling-video_v1_standard",
    feature = "endpoints_fal-ai_kling-video_v1_standard_effects"
))]
pub mod effects;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kling-video",
    feature = "endpoints_fal-ai_kling-video_v1",
    feature = "endpoints_fal-ai_kling-video_v1_standard",
    feature = "endpoints_fal-ai_kling-video_v1_standard_image-to-video"
))]
pub mod image_to_video;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kling-video",
    feature = "endpoints_fal-ai_kling-video_v1",
    feature = "endpoints_fal-ai_kling-video_v1_standard",
    feature = "endpoints_fal-ai_kling-video_v1_standard_text-to-video"
))]
pub mod text_to_video;
