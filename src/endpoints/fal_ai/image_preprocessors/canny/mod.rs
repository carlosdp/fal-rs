#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CannyOutput {
    /// Image with edges detected using the Canny algorithm
    pub image: Image,
}

/// Image Preprocessors
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn canny(params: CannyInput) -> FalRequest<CannyInput, CannyOutput> {
    FalRequest::new("fal-ai/image-preprocessors/canny", params)
}
