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
    

                /// Rundiffusion Photo Flux
/// 
/// RunDiffusion Photo Flux provides insane realism. With this enhancer, textures and skin details burst to life, turning your favorite prompts into vivid, lifelike creations. Recommended to keep it at 0.65 to 0.80 weight. Supports resolutions up to 1536x1536.
/// 
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
/// 
/// FLUX.1 [dev], next generation text-to-image model.
                pub fn juggernaut(params: PhotoLoraT2IInput) -> FalRequest<PhotoLoraT2IInput, Output> {
                    FalRequest::new(
                        "rundiffusion-fal/rundiffusion-photo-flux",
                        params
                    )
                }
                