#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct OCRBoundingBoxOutputWithLabels {
    /// Processed image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Option<Image>>,
    /// Results from the model
    pub results: OCRBoundingBox,
}

/// Florence-2 Large
///
/// Category: vision
/// Machine Type: A100
/// License Type: commercial
pub fn ocr_with_region(
    params: ImageInput,
) -> FalRequest<ImageInput, OCRBoundingBoxOutputWithLabels> {
    FalRequest::new("fal-ai/florence-2-large/ocr-with-region", params)
}
