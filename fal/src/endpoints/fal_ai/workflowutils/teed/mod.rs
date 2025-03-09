#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TeedOutput {
    /// The edge map./// The edge map./// {"content_type":"image/png","height":2048,"url":"https://storage.googleapis.com/falserverless/model_tests/workflow_utils/teed_output.png","width":1246}
    pub image: Image,
}

/// Image Preprocessors
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn teed(params: TeedInput) -> FalRequest<TeedInput, TeedOutput> {
    FalRequest::new("fal-ai/workflowutils/canny", params)
}
