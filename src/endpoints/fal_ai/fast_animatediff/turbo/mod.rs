#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_fast-animatediff",
    feature = "endpoints_fal-ai_fast-animatediff_turbo",
    feature = "endpoints_fal-ai_fast-animatediff_turbo_text-to-video"
))]
pub mod text_to_video;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_fast-animatediff",
    feature = "endpoints_fal-ai_fast-animatediff_turbo",
    feature = "endpoints_fal-ai_fast-animatediff_turbo_video-to-video"
))]
pub mod video_to_video;
