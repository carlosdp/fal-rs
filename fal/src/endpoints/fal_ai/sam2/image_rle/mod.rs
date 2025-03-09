#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SAM2RLEOutput {
    /// Run Length Encoding of the mask.
    pub rle: RleProperty,
}

/// Segment Anything Model 2
///
/// Category: image-to-image
/// Machine Type: A100
pub fn image_rle(params: SAM2ImageInput) -> FalRequest<SAM2ImageInput, SAM2RLEOutput> {
    FalRequest::new("fal-ai/sam2/image", params)
}
