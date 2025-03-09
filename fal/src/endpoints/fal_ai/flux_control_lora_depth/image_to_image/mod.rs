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
    

                /// FLUX.1 [dev] Control LoRA Depth
/// 
/// FLUX Control LoRA Depth is a high-performance endpoint that uses a control image to transfer structure to the generated image, using a depth map.
/// 
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX.1 [dev], next generation text-to-image model.
                pub fn image_to_image(params: ImageToImageInput) -> FalRequest<ImageToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-control-lora-depth",
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
    

                /// FLUX.1 [dev] Control LoRA Depth
/// 
/// FLUX Control LoRA Depth is a high-performance endpoint that uses a control image using a depth map to transfer structure to the generated image and another initial image to guide color.
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX.1 [dev], next generation text-to-image model.
                pub fn image_to_image(params: ImageToImageInput) -> FalRequest<ImageToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-control-lora-depth/image-to-image",
                        params
                    )
                }
                