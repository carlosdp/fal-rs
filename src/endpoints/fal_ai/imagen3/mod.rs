#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_imagen3",
    feature = "endpoints_fal-ai_imagen3_fast"
))]
pub mod fast;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_imagen3",
    feature = "endpoints_fal-ai_imagen3_subject-reference"
))]
pub mod subject_reference;

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
/// Generate images using Google's Imagen 3 model.
///
/// Imagen 3 is designed to generate high-quality images with enhanced detail, richer lighting,
/// and fewer artifacts. The model excels at:
/// - Capturing fine details and textures
/// - Rendering diverse art styles from photorealism to animation
/// - Understanding natural language prompts
/// - Maintaining high visual quality and composition
pub fn imagen3(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
    FalRequest::new("fal-ai/imagen3", params)
}
