#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct UpscaleOutput {
        /// Upscaled image
pub image: Image
    }
    

                /// Upscale Images
/// 
/// Upscale images by a given factor.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn esrgan(params: UpscaleInput) -> FalRequest<UpscaleInput, UpscaleOutput> {
                    FalRequest::new(
                        "fal-ai/esrgan",
                        params
                    )
                }
                