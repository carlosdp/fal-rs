#[cfg(any(
    feature = "endpoints_fal-ai_bria_background",
    feature = "endpoints_fal-ai_bria_background_remove",
    feature = "endpoints_fal-ai_bria_background_replace"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_bria_background",
        feature = "endpoints_fal-ai_bria_background_remove",
        feature = "endpoints_fal-ai_bria_background_replace"
    )))
)]
pub mod background;
#[cfg(any(feature = "endpoints_fal-ai_bria_eraser"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_bria_eraser"))))]
pub mod eraser;
#[cfg(any(feature = "endpoints_fal-ai_bria_expand"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_bria_expand"))))]
pub mod expand;
#[cfg(any(feature = "endpoints_fal-ai_bria_genfill"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_bria_genfill"))))]
pub mod genfill;
#[cfg(any(feature = "endpoints_fal-ai_bria_product-shot"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_bria_product-shot"))))]
pub mod product_shot;
#[cfg(any(feature = "endpoints_fal-ai_bria_reimagine"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_bria_reimagine"))))]
pub mod reimagine;
#[cfg(any(
    feature = "endpoints_fal-ai_bria_text-to-image",
    feature = "endpoints_fal-ai_bria_text-to-image_base",
    feature = "endpoints_fal-ai_bria_text-to-image_fast",
    feature = "endpoints_fal-ai_bria_text-to-image_hd"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_bria_text-to-image",
        feature = "endpoints_fal-ai_bria_text-to-image_base",
        feature = "endpoints_fal-ai_bria_text-to-image_fast",
        feature = "endpoints_fal-ai_bria_text-to-image_hd"
    )))
)]
pub mod text_to_image;
#[cfg(any(feature = "endpoints_fal-ai_bria_upscale"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_bria_upscale"))))]
pub mod upscale;
