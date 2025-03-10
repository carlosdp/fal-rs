#[cfg(any(
    feature = "endpoints_fal-ai_sa2va_v4b",
    feature = "endpoints_fal-ai_sa2va_v4b_image",
    feature = "endpoints_fal-ai_sa2va_v4b_video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_sa2va_v4b",
        feature = "endpoints_fal-ai_sa2va_v4b_image",
        feature = "endpoints_fal-ai_sa2va_v4b_video"
    )))
)]
pub mod v4b;
#[cfg(any(
    feature = "endpoints_fal-ai_sa2va_v8b",
    feature = "endpoints_fal-ai_sa2va_v8b_image",
    feature = "endpoints_fal-ai_sa2va_v8b_video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_sa2va_v8b",
        feature = "endpoints_fal-ai_sa2va_v8b_image",
        feature = "endpoints_fal-ai_sa2va_v8b_video"
    )))
)]
pub mod v8b;
