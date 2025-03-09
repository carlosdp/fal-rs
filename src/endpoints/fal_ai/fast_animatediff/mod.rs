#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_fast-animatediff",
    feature = "endpoints_fal-ai_fast-animatediff_text-to-video"
))]
pub mod text_to_video;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_fast-animatediff",
    feature = "endpoints_fal-ai_fast-animatediff_turbo"
))]
pub mod turbo;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_fast-animatediff",
    feature = "endpoints_fal-ai_fast-animatediff_video-to-video"
))]
pub mod video_to_video;
