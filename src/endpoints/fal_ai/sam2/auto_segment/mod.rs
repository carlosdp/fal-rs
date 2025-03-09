#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SAM2AutomaticSegmentationOutput {
    /// Combined segmentation mask.
    pub combined_mask: Image,
    /// Individual segmentation masks.
    pub individual_masks: Vec<Image>,
}

/// Segment Anything Model 2
///
/// Category: image-to-image
/// Machine Type: A100
pub fn auto_segment(
    params: SAM2AutomaticSegmentationInput,
) -> FalRequest<SAM2AutomaticSegmentationInput, SAM2AutomaticSegmentationOutput> {
    FalRequest::new("fal-ai/sam2/auto-segment", params)
}
