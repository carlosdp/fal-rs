#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct UpscaleOutput {
        /// The upscaled image.
pub image: Image
    }
    

                /// Recraft Clarity Upscale
/// 
/// Enhances a given raster image using 'clarity upscale' tool, increasing image resolution, making the image sharper and cleaner.
/// 
/// Category: image-to-image
/// 
/// 
/// 
/// Enhances a given raster image using 'clarity upscale' tool,
/// increasing image resolution, making the image sharper and cleaner.
                pub fn recraft_clarity_upscale(params: UpscaleInput) -> FalRequest<UpscaleInput, UpscaleOutput> {
                    FalRequest::new(
                        "fal-ai/recraft-clarity-upscale",
                        params
                    )
                }
                