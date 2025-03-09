#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_ffmpeg-api",
    feature = "endpoints_fal-ai_ffmpeg-api_compose"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_ffmpeg-api",
        feature = "endpoints_fal-ai_ffmpeg-api_compose"
    )))
)]
pub mod compose;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_ffmpeg-api",
    feature = "endpoints_fal-ai_ffmpeg-api_metadata"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_ffmpeg-api",
        feature = "endpoints_fal-ai_ffmpeg-api_metadata"
    )))
)]
pub mod metadata;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_ffmpeg-api",
    feature = "endpoints_fal-ai_ffmpeg-api_waveform"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_ffmpeg-api",
        feature = "endpoints_fal-ai_ffmpeg-api_waveform"
    )))
)]
pub mod waveform;
