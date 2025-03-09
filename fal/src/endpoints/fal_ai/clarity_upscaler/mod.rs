#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// The URL of the generated image.
pub image: Image,
/// The seed used to generate the image.
pub seed: i64,
/// The timings of the different steps in the workflow.
pub timings: Timings
    }
    

                /// Clarity Upscaler
/// 
/// Clarity upscaler for images with high fidelity.
/// 
/// Category: image-to-image
/// Machine Type: A100
                pub fn clarity_upscaler(params: Input) -> FalRequest<Input, Output> {
                    FalRequest::new(
                        "fal-ai/clarity-upscaler",
                        params
                    )
                }
                