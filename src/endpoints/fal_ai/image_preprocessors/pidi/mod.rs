#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PiDiOutput {
    /// Image with Pidi lines detected
    pub image: Image,
}

/// Image Preprocessors
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn pidi(params: PiDiInput) -> FalRequest<PiDiInput, PiDiOutput> {
    FalRequest::new("fal-ai/image-preprocessors/pidi", params)
}
