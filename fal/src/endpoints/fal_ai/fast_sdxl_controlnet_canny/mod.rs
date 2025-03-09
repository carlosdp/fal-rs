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
    

                /// ControlNet SDXL
/// 
/// Generate Images with ControlNet.
/// 
/// Category: text-to-image
                pub fn fast_sdxl_controlnet_canny(params: TextToImageControlNetInput) -> FalRequest<TextToImageControlNetInput, Output> {
                    FalRequest::new(
                        "fal-ai/fast-sdxl-controlnet-canny",
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
    

                /// ControlNet SDXL
/// 
/// Generate Images with ControlNet.
/// 
/// Category: image-to-image
                pub fn fast_sdxl_controlnet_canny(params: TextToImageControlNetInput) -> FalRequest<TextToImageControlNetInput, Output> {
                    FalRequest::new(
                        "fal-ai/fast-sdxl-controlnet-canny/image-to-image",
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
    

                /// ControlNet SDXL
/// 
/// Generate Images with ControlNet.
/// 
/// Category: image-to-image
                pub fn fast_sdxl_controlnet_canny(params: TextToImageControlNetInput) -> FalRequest<TextToImageControlNetInput, Output> {
                    FalRequest::new(
                        "fal-ai/fast-sdxl-controlnet-canny/inpainting",
                        params
                    )
                }
                