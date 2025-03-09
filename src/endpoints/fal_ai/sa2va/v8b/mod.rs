#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_sa2va",
    feature = "endpoints_fal-ai_sa2va_v8b",
    feature = "endpoints_fal-ai_sa2va_v8b_image"
))]
pub mod image;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_sa2va",
    feature = "endpoints_fal-ai_sa2va_v8b",
    feature = "endpoints_fal-ai_sa2va_v8b_video"
))]
pub mod video;
