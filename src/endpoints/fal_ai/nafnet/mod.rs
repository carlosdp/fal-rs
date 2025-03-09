#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_nafnet",
    feature = "endpoints_fal-ai_nafnet_deblur"
))]
pub mod deblur;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_nafnet",
    feature = "endpoints_fal-ai_nafnet_denoise"
))]
pub mod denoise;
