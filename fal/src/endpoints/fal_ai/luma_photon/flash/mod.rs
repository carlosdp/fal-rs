#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct T2IOutput {
        /// The generated image
pub images: Vec<File>
    }
    

                /// Luma Photon
/// 
/// Generate images from your prompts using Luma Photon. Photon is the most creative, personalizable, and intelligent visual models for creatives, bringing a step-function change in the cost of high-quality image generation.
/// 
/// Category: text-to-image
/// Machine Type: A100
                pub fn flash(params: TextToImageRequest) -> FalRequest<TextToImageRequest, T2IOutput> {
                    FalRequest::new(
                        "fal-ai/luma-photon",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct T2IOutput {
        /// The generated image
pub images: Vec<File>
    }
    

                /// Luma Photon Flash
/// 
/// Generate images from your prompts using Luma Photon Flash. Photon Flash is the most creative, personalizable, and intelligent visual models for creatives, bringing a step-function change in the cost of high-quality image generation.
/// 
/// Category: text-to-image
/// Machine Type: A100
                pub fn flash(params: TextToImageRequest) -> FalRequest<TextToImageRequest, T2IOutput> {
                    FalRequest::new(
                        "fal-ai/luma-photon/flash",
                        params
                    )
                }
                