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
    

                /// Stable Cascade
/// 
/// Stable Cascade: Image generation on a smaller & cheaper latent space.
/// 
/// Category: text-to-image
/// Machine Type: A100
/// License Type: research
                pub fn sote_diffusion(params: SoteDiffusionInput) -> FalRequest<SoteDiffusionInput, Output> {
                    FalRequest::new(
                        "fal-ai/stable-cascade",
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
    

                /// SoteDiffusion
/// 
/// Anime finetune of Würstchen V3.
/// 
/// Category: text-to-image
/// Machine Type: A100
/// License Type: research
                pub fn sote_diffusion(params: SoteDiffusionInput) -> FalRequest<SoteDiffusionInput, Output> {
                    FalRequest::new(
                        "fal-ai/stable-cascade/sote-diffusion",
                        params
                    )
                }
                