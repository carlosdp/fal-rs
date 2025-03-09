#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_ideogram",
    feature = "endpoints_fal-ai_ideogram_upscale"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_ideogram",
        feature = "endpoints_fal-ai_ideogram_upscale"
    )))
)]
pub mod upscale;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_ideogram",
    feature = "endpoints_fal-ai_ideogram_v2"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_ideogram",
        feature = "endpoints_fal-ai_ideogram_v2"
    )))
)]
pub mod v2;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_ideogram",
    feature = "endpoints_fal-ai_ideogram_v2a"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_ideogram",
        feature = "endpoints_fal-ai_ideogram_v2a"
    )))
)]
pub mod v2a;
