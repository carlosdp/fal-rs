#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct IllusionDiffusionOutput {
        /// The generated image file info.
pub image: Image,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64
    }
    

                /// Illusion Diffusion
/// 
/// Create illusions conditioned on image.
/// 
/// Category: text-to-image
/// Machine Type: A6000
                pub fn illusion_diffusion(params: IllusionDiffusionInput) -> FalRequest<IllusionDiffusionInput, IllusionDiffusionOutput> {
                    FalRequest::new(
                        "fal-ai/illusion-diffusion",
                        params
                    )
                }
                