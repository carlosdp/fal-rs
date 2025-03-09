#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageOutput {
    /// Whether the generated images contain NSFW concepts.
    pub has_nsfw_concepts: Vec<bool>,
    /// The generated images
    /// [{"content_type":"image/jpeg","height":768,"url":"https://v3.fal.media/files/rabbit/pBwaEZysJhnstKWEHGpLc.png","width":1024}]
    pub images: Vec<Image>,
    /// The prompt used for generating the image.
    pub prompt: String,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
    pub timings: Timings,
}

/// Lumina Image 2
///
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
pub fn v2(params: TextToImageInput) -> FalRequest<TextToImageInput, ImageOutput> {
    FalRequest::new("fal-ai/lumina-image/v2", params)
}
