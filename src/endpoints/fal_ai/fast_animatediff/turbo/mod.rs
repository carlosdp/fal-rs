#[cfg(any(feature = "endpoints_fal-ai_fast-animatediff_turbo_text-to-video"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_fast-animatediff_turbo_text-to-video")))
)]
pub mod text_to_video;
#[cfg(any(feature = "endpoints_fal-ai_fast-animatediff_turbo_video-to-video"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_fast-animatediff_turbo_video-to-video")))
)]
pub mod video_to_video;
