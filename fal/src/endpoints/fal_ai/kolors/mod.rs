#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod image_to_image;


                
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
    

                /// Kolors
/// 
/// Photorealistic Text-to-Image
/// 
/// Category: text-to-image
/// Machine Type: A6000
/// License Type: commercial
                pub fn kolors(params: KolorsInput) -> FalRequest<KolorsInput, Output> {
                    FalRequest::new(
                        "fal-ai/kolors",
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
    

                /// Kolors Image to Image
/// 
/// Photorealistic Image-to-Image
/// 
/// Category: image-to-image
/// Machine Type: A6000
/// License Type: commercial
                pub fn kolors(params: KolorsInput) -> FalRequest<KolorsInput, Output> {
                    FalRequest::new(
                        "fal-ai/kolors/image-to-image",
                        params
                    )
                }
                