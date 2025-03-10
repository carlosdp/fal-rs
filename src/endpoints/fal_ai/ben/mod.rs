#[cfg(any(
    feature = "endpoints_fal-ai_ben_v2",
    feature = "endpoints_fal-ai_ben_v2_image",
    feature = "endpoints_fal-ai_ben_v2_video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_ben_v2",
        feature = "endpoints_fal-ai_ben_v2_image",
        feature = "endpoints_fal-ai_ben_v2_video"
    )))
)]
pub mod v2;
