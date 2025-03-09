#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PixArtSigmaOutput {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image files info.
pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
/// The timings of the different steps of the generation process.
pub timings: Timings
    }
    

                /// PixArt-Σ
/// 
/// Weak-to-Strong Training of Diffusion Transformer for 4K Text-to-Image Generation
/// 
/// Category: text-to-image
/// Machine Type: A6000
                pub fn pixart_sigma(params: PixArtSigmaInput) -> FalRequest<PixArtSigmaInput, PixArtSigmaOutput> {
                    FalRequest::new(
                        "fal-ai/pixart-sigma",
                        params
                    )
                }
                