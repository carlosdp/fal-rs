#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod image_to_image;
pub mod redux;


                
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
    

                /// Juggernaut Flux Base
/// 
/// Juggernaut Base Flux by RunDiffusion is a drop-in replacement for Flux [Dev] that delivers sharper details, richer colors, and enhanced realism, while instantly boosting LoRAs and LyCORIS with full compatibility.
/// 
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
/// 
/// Finetuned version of FLUX.1 [dev], improving quality while preserving lora support.
                pub fn base(params: DevTextToImageInput) -> FalRequest<DevTextToImageInput, Output> {
                    FalRequest::new(
                        "rundiffusion-fal/juggernaut-flux/base",
                        params
                    )
                }
                
                
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
    

                /// Juggernaut Flux Lightning
/// 
/// Juggernaut Lightning Flux by RunDiffusion provides blazing-fast, high-quality images rendered at five times the speed of Flux. Perfect for mood boards and mass ideation, this model excels in both realism and prompt adherence.
/// 
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
/// 
/// Finetuned version of FLUX.1 [dev], improving quality while preserving lora support.
                pub fn base(params: DevTextToImageInput) -> FalRequest<DevTextToImageInput, Output> {
                    FalRequest::new(
                        "rundiffusion-fal/juggernaut-flux/lightning",
                        params
                    )
                }
                
                
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
    

                /// Juggernaut Flux Pro
/// 
/// Juggernaut Pro Flux by RunDiffusion is the flagship Juggernaut model rivaling some of the most advanced image models available, often surpassing them in realism. It combines Juggernaut Base with RunDiffusion Photo and features enhancements like reduced background blurriness.
/// 
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
/// 
/// Finetuned version of FLUX.1 [dev], improving quality while preserving lora support.
                pub fn base(params: DevTextToImageInput) -> FalRequest<DevTextToImageInput, Output> {
                    FalRequest::new(
                        "rundiffusion-fal/juggernaut-flux/pro",
                        params
                    )
                }
                