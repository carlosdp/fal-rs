#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SamOutput {
    /// Combined image of all detected masks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Option<Image>>,
}

/// Midas Depth Estimation
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn sam(params: SamInput) -> FalRequest<SamInput, SamOutput> {
    FalRequest::new("fal-ai/imageutils/depth", params)
}
