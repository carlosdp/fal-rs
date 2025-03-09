#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransparentImageToMaskOutput {
    /// The mask/// The mask/// {"content_type":"image/png","height":700,"url":"https://storage.googleapis.com/falserverless/model_tests/workflow_utils/transparent_image_to_mask_output.png","width":610}
    pub image: Image,
}

/// Image Preprocessors
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn transparent_image_to_mask(
    params: TransparentImageToMaskInput,
) -> FalRequest<TransparentImageToMaskInput, TransparentImageToMaskOutput> {
    FalRequest::new("fal-ai/workflowutils/transparent-image-to-mask", params)
}
