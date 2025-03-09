#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSizeOutput {
    /// Image size
    /// {"height":700,"width":610}
    pub image_size: HashMap<String, serde_json::Value>,
}

/// Image Preprocessors
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn image_size(params: ImageInput) -> FalRequest<ImageInput, ImageSizeOutput> {
    FalRequest::new("fal-ai/workflowutils/image-size", params)
}
