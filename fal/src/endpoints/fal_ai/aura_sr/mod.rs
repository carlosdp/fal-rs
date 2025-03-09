#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Upscaled image
pub image: Image,
/// Timings for each step in the pipeline.
pub timings: Timings
    }
    

                /// AuraSR
/// 
/// Upscale your images with AuraSR.
/// 
/// Category: image-to-image
/// Machine Type: A100
                pub fn aura_sr(params: Input) -> FalRequest<Input, Output> {
                    FalRequest::new(
                        "fal-ai/aura-sr",
                        params
                    )
                }
                