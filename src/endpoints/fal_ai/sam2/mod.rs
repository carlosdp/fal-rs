#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_sam2",
    feature = "endpoints_fal-ai_sam2_auto-segment"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_sam2",
        feature = "endpoints_fal-ai_sam2_auto-segment"
    )))
)]
pub mod auto_segment;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_sam2",
    feature = "endpoints_fal-ai_sam2_image"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_sam2",
        feature = "endpoints_fal-ai_sam2_image"
    )))
)]
pub mod image;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_sam2",
    feature = "endpoints_fal-ai_sam2_image-rle"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_sam2",
        feature = "endpoints_fal-ai_sam2_image-rle"
    )))
)]
pub mod image_rle;
