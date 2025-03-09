#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// The URL of the generated image.
pub image: Image,
/// The seed used to generate the image.
pub seed: i64
    }
    

                /// Layer Diffusion XL
/// 
/// SDXL with an alpha channel.
/// 
/// Category: text-to-image
/// Machine Type: A100
                pub fn layer_diffusion(params: Input) -> FalRequest<Input, Output> {
                    FalRequest::new(
                        "fal-ai/layer-diffusion",
                        params
                    )
                }
                