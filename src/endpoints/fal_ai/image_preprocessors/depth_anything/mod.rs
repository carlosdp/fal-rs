#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_image-preprocessors",
    feature = "endpoints_fal-ai_image-preprocessors_depth-anything",
    feature = "endpoints_fal-ai_image-preprocessors_depth-anything_v2"
))]
pub mod v2;
