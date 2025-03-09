#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_imageutils",
    feature = "endpoints_fal-ai_imageutils_depth"
))]
pub mod depth;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_imageutils",
    feature = "endpoints_fal-ai_imageutils_esrgan"
))]
pub mod esrgan;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_imageutils",
    feature = "endpoints_fal-ai_imageutils_marigold-depth"
))]
pub mod marigold_depth;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_imageutils",
    feature = "endpoints_fal-ai_imageutils_nsfw"
))]
pub mod nsfw;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_imageutils",
    feature = "endpoints_fal-ai_imageutils_rembg"
))]
pub mod rembg;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_imageutils",
    feature = "endpoints_fal-ai_imageutils_sam"
))]
pub mod sam;
