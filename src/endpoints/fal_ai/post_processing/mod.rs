#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessedOutput {
    /// The processed images
    pub images: Vec<Image>,
}

/// Post Processing
///
/// Category: image-to-image
pub fn post_processing(
    params: ImageProcessingInput,
) -> FalRequest<ImageProcessingInput, ProcessedOutput> {
    FalRequest::new("fal-ai/post-processing", params)
}
