#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Upscaled image
pub image: Image
    }
    

                /// DRCT-Super-Resolution
/// 
/// Upscale your images with DRCT-Super-Resolution.
/// 
/// Category: image-to-image
/// Machine Type: A100
                pub fn drct_super_resolution(params: Input) -> FalRequest<Input, Output> {
                    FalRequest::new(
                        "fal-ai/drct-super-resolution",
                        params
                    )
                }
                