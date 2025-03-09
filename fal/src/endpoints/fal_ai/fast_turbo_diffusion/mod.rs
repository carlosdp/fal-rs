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
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// Stable Diffusion Turbo (v1.5/XL)
/// 
/// Run SDXL at the speed of light
/// 
/// Category: text-to-image
/// Machine Type: A6000
                pub fn fast_turbo_diffusion(params: TextToImageTurboInput) -> FalRequest<TextToImageTurboInput, Output> {
                    FalRequest::new(
                        "fal-ai/fast-turbo-diffusion",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image files info.
pub images: Vec<Image>,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// Stable Diffusion Turbo (v1.5/XL)
/// 
/// Run SDXL at the speed of light
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn fast_turbo_diffusion(params: TextToImageTurboInput) -> FalRequest<TextToImageTurboInput, Output> {
                    FalRequest::new(
                        "fal-ai/fast-turbo-diffusion/image-to-image",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image files info.
pub images: Vec<Image>,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// Stable Diffusion Turbo (v1.5/XL)
/// 
/// Run SDXL at the speed of light
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn fast_turbo_diffusion(params: TextToImageTurboInput) -> FalRequest<TextToImageTurboInput, Output> {
                    FalRequest::new(
                        "fal-ai/fast-turbo-diffusion/inpainting",
                        params
                    )
                }
                