#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub images: Vec<File>,
    /// Seed used for generation/// Seed used for generation/// 42
    pub seed: i64,
}

/// Imagen3
///
/// Category: text-to-image
///
///
///
/// Generate images using Google's Imagen 3 Fast model for lower latency.
///
/// A faster version of Imagen 3 that maintains high quality while providing:
/// - Quicker generation times
/// - Support for diverse art styles
/// - Good prompt understanding
/// - Reliable text rendering
pub fn fast(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
    FalRequest::new("fal-ai/imagen3", params)
}
