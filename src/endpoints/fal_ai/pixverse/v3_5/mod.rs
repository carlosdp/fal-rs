#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_pixverse",
    feature = "endpoints_fal-ai_pixverse_v3-5",
    feature = "endpoints_fal-ai_pixverse_v3-5_image-to-video"
))]
pub mod image_to_video;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_pixverse",
    feature = "endpoints_fal-ai_pixverse_v3-5",
    feature = "endpoints_fal-ai_pixverse_v3-5_text-to-video"
))]
pub mod text_to_video;
