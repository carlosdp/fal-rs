#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_lumina-image",
    feature = "endpoints_fal-ai_lumina-image_v2"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_lumina-image",
        feature = "endpoints_fal-ai_lumina-image_v2"
    )))
)]
pub mod v2;
