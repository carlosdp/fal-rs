#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageOutput {
    /// The segmented output image
    pub image: File,
}

/// EVF-SAM2 Segmentation
///
/// Category: image-to-image
pub fn evf_sam(params: ImageInput) -> FalRequest<ImageInput, ImageOutput> {
    FalRequest::new("fal-ai/evf-sam", params)
}
