#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
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
    

                /// Stable Diffusion 3.5 Large
/// 
/// Stable Diffusion 3.5 Large is a Multimodal Diffusion Transformer (MMDiT) text-to-image model that features improved performance in image quality, typography, complex prompt understanding, and resource-efficiency.
/// 
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
                pub fn image_to_image(params: ImageToImageInput) -> FalRequest<ImageToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/stable-diffusion-v35-large",
                        params
                    )
                }
                