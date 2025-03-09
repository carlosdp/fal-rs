#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct GrowMaskOutput {
    /// The mask/// The mask/// {"content_type":"image/png","height":700,"url":"https://storage.googleapis.com/falserverless/model_tests/workflow_utils/grow_output.png","width":610}
    pub image: Image,
}

/// Image Preprocessors
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn grow_mask(params: GrowMaskInput) -> FalRequest<GrowMaskInput, GrowMaskOutput> {
    FalRequest::new("fal-ai/workflowutils/grow-mask", params)
}
