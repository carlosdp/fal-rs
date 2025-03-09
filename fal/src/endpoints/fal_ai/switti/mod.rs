#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod 512;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SwittiOutput {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated images/// The generated images/// [{"content_type":"image/jpeg","height":1024,"url":"https://fal.media/files/lion/JpgBX7w379jHteLeeNsM5.jpeg","width":1024}]

pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// Switti 1024
/// 
/// Switti is a scale-wise transformer for fast text-to-image generation that outperforms existing T2I AR models and competes with state-of-the-art T2I diffusion models while being faster than distilled diffusion models.
/// 
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
                pub fn switti(params: TextToImageInput) -> FalRequest<TextToImageInput, SwittiOutput> {
                    FalRequest::new(
                        "fal-ai/switti",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SwittiOutput {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated images/// The generated images/// [{"content_type":"image/jpeg","height":1024,"url":"https://fal.media/files/lion/JpgBX7w379jHteLeeNsM5.jpeg","width":1024}]

pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// Switti 512
/// 
/// Switti is a scale-wise transformer for fast text-to-image generation that outperforms existing T2I AR models and competes with state-of-the-art T2I diffusion models while being faster than distilled diffusion models.
/// 
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
                pub fn switti(params: TextToImageInput) -> FalRequest<TextToImageInput, SwittiOutput> {
                    FalRequest::new(
                        "fal-ai/switti/512",
                        params
                    )
                }
                