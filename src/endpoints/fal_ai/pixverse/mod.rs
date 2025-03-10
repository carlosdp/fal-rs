#[cfg(any(
    feature = "endpoints_fal-ai_pixverse_v3-5",
    feature = "endpoints_fal-ai_pixverse_v3-5_image-to-video",
    feature = "endpoints_fal-ai_pixverse_v3-5_image-to-video_fast",
    feature = "endpoints_fal-ai_pixverse_v3-5_text-to-video",
    feature = "endpoints_fal-ai_pixverse_v3-5_text-to-video_fast"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_pixverse_v3-5",
        feature = "endpoints_fal-ai_pixverse_v3-5_image-to-video",
        feature = "endpoints_fal-ai_pixverse_v3-5_image-to-video_fast",
        feature = "endpoints_fal-ai_pixverse_v3-5_text-to-video",
        feature = "endpoints_fal-ai_pixverse_v3-5_text-to-video_fast"
    )))
)]
pub mod v3_5;
