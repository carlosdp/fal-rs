#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct HEDOutput {
    /// Image with lines detected using the HED detector
    pub image: Image,
}

/// Image Preprocessors
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn hed(params: HEDInput) -> FalRequest<HEDInput, HEDOutput> {
    FalRequest::new("fal-ai/image-preprocessors/hed", params)
}
