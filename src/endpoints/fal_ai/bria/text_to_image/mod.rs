#[cfg(any(feature = "endpoints_fal-ai_bria_text-to-image_base"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_bria_text-to-image_base")))
)]
pub mod base;
#[cfg(any(feature = "endpoints_fal-ai_bria_text-to-image_fast"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_bria_text-to-image_fast")))
)]
pub mod fast;
#[cfg(any(feature = "endpoints_fal-ai_bria_text-to-image_hd"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_bria_text-to-image_hd")))
)]
pub mod hd;
