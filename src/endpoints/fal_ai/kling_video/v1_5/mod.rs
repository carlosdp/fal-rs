#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kling-video",
    feature = "endpoints_fal-ai_kling-video_v1-5",
    feature = "endpoints_fal-ai_kling-video_v1-5_pro"
))]
pub mod pro;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kling-video",
    feature = "endpoints_fal-ai_kling-video_v1-5",
    feature = "endpoints_fal-ai_kling-video_v1-5_standard"
))]
pub mod standard;
