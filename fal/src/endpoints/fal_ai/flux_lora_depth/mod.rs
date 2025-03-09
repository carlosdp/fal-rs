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
    

                /// FLUX.1 [dev] Depth with LoRAs
/// 
/// Generate high-quality images from depth maps using Flux.1 [dev] depth estimation model. The model produces accurate depth representations for scene understanding and 3D visualization.
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX.1 [dev], next generation text-to-image model.
                pub fn flux_lora_depth(params: DepthInput) -> FalRequest<DepthInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-lora-depth",
                        params
                    )
                }
                