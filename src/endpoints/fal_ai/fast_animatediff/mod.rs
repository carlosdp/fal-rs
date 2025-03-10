#[cfg(any(feature = "endpoints_fal-ai_fast-animatediff_text-to-video"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_fast-animatediff_text-to-video")))
)]
pub mod text_to_video;
#[cfg(any(
    feature = "endpoints_fal-ai_fast-animatediff_turbo",
    feature = "endpoints_fal-ai_fast-animatediff_turbo_text-to-video",
    feature = "endpoints_fal-ai_fast-animatediff_turbo_video-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_fast-animatediff_turbo",
        feature = "endpoints_fal-ai_fast-animatediff_turbo_text-to-video",
        feature = "endpoints_fal-ai_fast-animatediff_turbo_video-to-video"
    )))
)]
pub mod turbo;
#[cfg(any(feature = "endpoints_fal-ai_fast-animatediff_video-to-video"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_fast-animatediff_video-to-video")))
)]
pub mod video_to_video;
