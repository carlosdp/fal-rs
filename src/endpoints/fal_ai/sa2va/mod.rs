#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_sa2va",
    feature = "endpoints_fal-ai_sa2va_v4b"
))]
pub mod v4b;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_sa2va",
    feature = "endpoints_fal-ai_sa2va_v8b"
))]
pub mod v8b;
