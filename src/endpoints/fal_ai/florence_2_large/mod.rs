#[cfg(any(feature = "endpoints_fal-ai_florence-2-large_caption"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_florence-2-large_caption")))
)]
pub mod caption;
#[cfg(any(feature = "endpoints_fal-ai_florence-2-large_caption-to-phrase-grounding"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_florence-2-large_caption-to-phrase-grounding")))
)]
pub mod caption_to_phrase_grounding;
#[cfg(any(feature = "endpoints_fal-ai_florence-2-large_dense-region-caption"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_florence-2-large_dense-region-caption")))
)]
pub mod dense_region_caption;
#[cfg(any(feature = "endpoints_fal-ai_florence-2-large_detailed-caption"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_florence-2-large_detailed-caption")))
)]
pub mod detailed_caption;
#[cfg(any(feature = "endpoints_fal-ai_florence-2-large_more-detailed-caption"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_florence-2-large_more-detailed-caption")))
)]
pub mod more_detailed_caption;
#[cfg(any(feature = "endpoints_fal-ai_florence-2-large_object-detection"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_florence-2-large_object-detection")))
)]
pub mod object_detection;
#[cfg(any(feature = "endpoints_fal-ai_florence-2-large_ocr"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_florence-2-large_ocr")))
)]
pub mod ocr;
#[cfg(any(feature = "endpoints_fal-ai_florence-2-large_ocr-with-region"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_florence-2-large_ocr-with-region")))
)]
pub mod ocr_with_region;
#[cfg(any(feature = "endpoints_fal-ai_florence-2-large_open-vocabulary-detection"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_florence-2-large_open-vocabulary-detection")))
)]
pub mod open_vocabulary_detection;
#[cfg(any(feature = "endpoints_fal-ai_florence-2-large_referring-expression-segmentation"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_florence-2-large_referring-expression-segmentation"
    )))
)]
pub mod referring_expression_segmentation;
#[cfg(any(feature = "endpoints_fal-ai_florence-2-large_region-proposal"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_florence-2-large_region-proposal")))
)]
pub mod region_proposal;
#[cfg(any(feature = "endpoints_fal-ai_florence-2-large_region-to-category"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_florence-2-large_region-to-category")))
)]
pub mod region_to_category;
#[cfg(any(feature = "endpoints_fal-ai_florence-2-large_region-to-description"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_florence-2-large_region-to-description")))
)]
pub mod region_to_description;
#[cfg(any(feature = "endpoints_fal-ai_florence-2-large_region-to-segmentation"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_florence-2-large_region-to-segmentation")))
)]
pub mod region_to_segmentation;
