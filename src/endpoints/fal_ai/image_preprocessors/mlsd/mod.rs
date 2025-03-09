#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MLSDOutput {
    /// Image with lines detected using the MLSD detector
    pub image: Image,
}

/// Image Preprocessors
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn mlsd(params: MLSDInput) -> FalRequest<MLSDInput, MLSDOutput> {
    FalRequest::new("fal-ai/image-preprocessors/mlsd", params)
}
