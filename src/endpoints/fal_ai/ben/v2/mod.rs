#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_ben",
    feature = "endpoints_fal-ai_ben_v2",
    feature = "endpoints_fal-ai_ben_v2_image"
))]
pub mod image;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_ben",
    feature = "endpoints_fal-ai_ben_v2",
    feature = "endpoints_fal-ai_ben_v2_video"
))]
pub mod video;
