#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScribbleOutput {
    /// Image with lines detected using the Scribble detector
    pub image: Image,
}

/// Image Preprocessors
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn scribble(params: ScribbleInput) -> FalRequest<ScribbleInput, ScribbleOutput> {
    FalRequest::new("fal-ai/image-preprocessors/depth-anything/v2", params)
}
