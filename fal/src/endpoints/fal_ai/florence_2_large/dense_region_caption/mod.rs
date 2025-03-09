#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct BoundingBoxOutputWithLabels {
    /// Processed image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Option<Image>>,
    /// Results from the model
    pub results: BoundingBoxes,
}

/// Florence-2 Large
///
/// Category: vision
/// Machine Type: A100
/// License Type: commercial
pub fn dense_region_caption(
    params: ImageInput,
) -> FalRequest<ImageInput, BoundingBoxOutputWithLabels> {
    FalRequest::new("fal-ai/florence-2-large/caption", params)
}
