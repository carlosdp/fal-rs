#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

pub mod image_to_image;
pub mod inpaint;

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputParameters {
    /// The latents saved for debugging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_latents: Option<Option<File>>,
    /// The latents saved for debugging per pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_per_pass_latents: Option<Option<File>>,
    /// Whether the generated images contain NSFW concepts.
    pub has_nsfw_concepts: Vec<bool>,
    /// The generated image files info.
    pub images: Vec<Image>,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
}

/// Stable Diffusion with LoRAs
///
/// Category: text-to-image
/// Machine Type: A100
pub fn lora(params: TextToImageInput) -> FalRequest<TextToImageInput, OutputParameters> {
    FalRequest::new("fal-ai/lora", params)
}
