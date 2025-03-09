#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_bria",
    feature = "endpoints_fal-ai_bria_text-to-image",
    feature = "endpoints_fal-ai_bria_text-to-image_base"
))]
pub mod base;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_bria",
    feature = "endpoints_fal-ai_bria_text-to-image",
    feature = "endpoints_fal-ai_bria_text-to-image_fast"
))]
pub mod fast;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_bria",
    feature = "endpoints_fal-ai_bria_text-to-image",
    feature = "endpoints_fal-ai_bria_text-to-image_hd"
))]
pub mod hd;
