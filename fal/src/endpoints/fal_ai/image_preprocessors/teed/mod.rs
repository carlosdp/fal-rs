#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TeeDOutput {
    /// Image with TeeD lines detected
    pub image: Image,
}

/// Image Preprocessors
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn teed(params: TeeDInput) -> FalRequest<TeeDInput, TeeDOutput> {
    FalRequest::new("fal-ai/image-preprocessors/depth-anything/v2", params)
}
