#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_image-preprocessors",
    feature = "endpoints_fal-ai_image-preprocessors_canny"
))]
pub mod canny;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_image-preprocessors",
    feature = "endpoints_fal-ai_image-preprocessors_depth-anything"
))]
pub mod depth_anything;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_image-preprocessors",
    feature = "endpoints_fal-ai_image-preprocessors_hed"
))]
pub mod hed;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_image-preprocessors",
    feature = "endpoints_fal-ai_image-preprocessors_lineart"
))]
pub mod lineart;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_image-preprocessors",
    feature = "endpoints_fal-ai_image-preprocessors_midas"
))]
pub mod midas;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_image-preprocessors",
    feature = "endpoints_fal-ai_image-preprocessors_mlsd"
))]
pub mod mlsd;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_image-preprocessors",
    feature = "endpoints_fal-ai_image-preprocessors_pidi"
))]
pub mod pidi;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_image-preprocessors",
    feature = "endpoints_fal-ai_image-preprocessors_sam"
))]
pub mod sam;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_image-preprocessors",
    feature = "endpoints_fal-ai_image-preprocessors_scribble"
))]
pub mod scribble;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_image-preprocessors",
    feature = "endpoints_fal-ai_image-preprocessors_teed"
))]
pub mod teed;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_image-preprocessors",
    feature = "endpoints_fal-ai_image-preprocessors_zoe"
))]
pub mod zoe;
