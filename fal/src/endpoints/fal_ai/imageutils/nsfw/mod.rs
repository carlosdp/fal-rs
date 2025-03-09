#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct NSFWImageDetectionOutput {
    /// The probability of the image being NSFW.
    pub nsfw_probability: f64,
}

/// Midas Depth Estimation
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn nsfw(
    params: NSFWImageDetectionInput,
) -> FalRequest<NSFWImageDetectionInput, NSFWImageDetectionOutput> {
    FalRequest::new("fal-ai/imageutils/depth", params)
}
