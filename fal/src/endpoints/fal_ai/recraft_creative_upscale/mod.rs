#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct UpscaleOutput {
        /// The upscaled image.
pub image: Image
    }
    

                /// Recraft Creative Upscale
/// 
/// Enhances a given raster image using 'creative upscale' tool, boosting resolution with a focus on refining small details and faces.
/// 
/// Category: image-to-image
/// 
/// 
/// 
/// Enhances a given raster image using 'clarity upscale' tool,
/// increasing image resolution, making the image sharper and cleaner.
                pub fn recraft_creative_upscale(params: UpscaleInput) -> FalRequest<UpscaleInput, UpscaleOutput> {
                    FalRequest::new(
                        "fal-ai/recraft-creative-upscale",
                        params
                    )
                }
                