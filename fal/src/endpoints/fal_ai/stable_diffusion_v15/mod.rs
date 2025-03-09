#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod image_to_image;
pub mod inpainting;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image files info.
pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// Stable Diffusion v1.5
/// 
/// Stable Diffusion v1.5
/// 
/// Category: text-to-image
/// Machine Type: A6000
                pub fn stable_diffusion_v15(params: TextToImageSD15Input) -> FalRequest<TextToImageSD15Input, Output> {
                    FalRequest::new(
                        "fal-ai/stable-diffusion-v15",
                        params
                    )
                }
                