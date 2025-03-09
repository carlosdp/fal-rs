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
    

                /// SDXL ControlNet Union
/// 
/// An efficent SDXL multi-controlnet text-to-image model.
/// 
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
                pub fn image_to_image(params: ImageToImageControlNetUnionInput) -> FalRequest<ImageToImageControlNetUnionInput, Output> {
                    FalRequest::new(
                        "fal-ai/sdxl-controlnet-union",
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
    

                /// SDXL ControlNet Union
/// 
/// An efficent SDXL multi-controlnet image-to-image model.
/// 
/// Category: image-to-image
/// Machine Type: A100
/// License Type: commercial
                pub fn image_to_image(params: ImageToImageControlNetUnionInput) -> FalRequest<ImageToImageControlNetUnionInput, Output> {
                    FalRequest::new(
                        "fal-ai/sdxl-controlnet-union/image-to-image",
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
    

                /// SDXL ControlNet Union
/// 
/// An efficent SDXL multi-controlnet inpainting model.
/// 
/// Category: image-to-image
/// Machine Type: A100
/// License Type: commercial
                pub fn image_to_image(params: ImageToImageControlNetUnionInput) -> FalRequest<ImageToImageControlNetUnionInput, Output> {
                    FalRequest::new(
                        "fal-ai/sdxl-controlnet-union/inpainting",
                        params
                    )
                }
                