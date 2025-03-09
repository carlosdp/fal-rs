#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SAM2ImageOutput {
    /// Segmented image.
    pub image: Image,
}

/// Segment Anything Model 2
///
/// Category: image-to-image
/// Machine Type: A100
pub fn image(params: SAM2ImageInput) -> FalRequest<SAM2ImageInput, SAM2ImageOutput> {
    FalRequest::new("fal-ai/sam2/image", params)
}
