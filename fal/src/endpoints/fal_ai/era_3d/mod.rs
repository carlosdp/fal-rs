#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Era3DOutput {
    /// Images with background removed
    pub images: Vec<Image>,
    /// Normal images with background removed
    pub normal_images: Vec<Image>,
    /// Seed used for random number generation
    pub seed: i64,
}

/// Era 3D
///
/// Category: image-to-image
/// Machine Type: A100
/// License Type: commercial
pub fn era_3d(params: Era3DInput) -> FalRequest<Era3DInput, Era3DOutput> {
    FalRequest::new("fal-ai/era-3d", params)
}
