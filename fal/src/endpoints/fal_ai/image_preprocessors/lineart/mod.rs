#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct LineartOutput {
    /// Image with edges detected using the Canny algorithm
    pub image: Image,
}

/// Image Preprocessors
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn lineart(params: LineartInput) -> FalRequest<LineartInput, LineartOutput> {
    FalRequest::new("fal-ai/image-preprocessors/lineart", params)
}
