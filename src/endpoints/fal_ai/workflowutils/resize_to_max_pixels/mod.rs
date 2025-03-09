#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageOutput {
    /// The output image/// The output image/// {"content_type":"image/png","height":700,"url":"https://storage.googleapis.com/falserverless/model_tests/workflow_utils/invert_mask_output.png","width":610}
    pub image: Image,
}

/// Image Preprocessors
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn resize_to_max_pixels(
    params: ResizeToPixelsInput,
) -> FalRequest<ResizeToPixelsInput, ImageOutput> {
    FalRequest::new("fal-ai/workflowutils/resize-to-max-pixels", params)
}
