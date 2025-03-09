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
    

                /// FLUX.1 [dev] with LoRAs
/// 
/// Super fast endpoint for the FLUX.1 [dev] model with LoRA support, enabling rapid and high-quality image generation using pre-trained LoRA adaptations for personalization, specific styles, brand identities, and product-specific outputs.
/// 
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX.1 [dev], next generation text-to-image model.
                pub fn flux_lora(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-lora",
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
    

                /// FLUX.1 [dev] Inpainting with LoRAs
/// 
/// Super fast endpoint for the FLUX.1 [dev] inpainting model with LoRA support, enabling rapid and high-quality image inpaingting using pre-trained LoRA adaptations for personalization, specific styles, brand identities, and product-specific outputs.
/// 
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX.1 [dev], next generation text-to-image model.
                pub fn flux_lora(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-lora/inpainting",
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
    

                /// FLUX.1 [dev] with LoRAs
/// 
/// FLUX LoRA Image-to-Image is a high-performance endpoint that transforms existing images using FLUX models, leveraging LoRA adaptations to enable rapid and precise image style transfer, modifications, and artistic variations.
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX.1 [dev], next generation text-to-image model.
                pub fn flux_lora(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-lora/image-to-image",
                        params
                    )
                }
                