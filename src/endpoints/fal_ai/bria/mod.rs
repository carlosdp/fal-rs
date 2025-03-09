#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_bria",
    feature = "endpoints_fal-ai_bria_background"
))]
pub mod background;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_bria",
    feature = "endpoints_fal-ai_bria_eraser"
))]
pub mod eraser;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_bria",
    feature = "endpoints_fal-ai_bria_expand"
))]
pub mod expand;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_bria",
    feature = "endpoints_fal-ai_bria_genfill"
))]
pub mod genfill;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_bria",
    feature = "endpoints_fal-ai_bria_product-shot"
))]
pub mod product_shot;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_bria",
    feature = "endpoints_fal-ai_bria_reimagine"
))]
pub mod reimagine;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_bria",
    feature = "endpoints_fal-ai_bria_text-to-image"
))]
pub mod text_to_image;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_bria",
    feature = "endpoints_fal-ai_bria_upscale"
))]
pub mod upscale;
