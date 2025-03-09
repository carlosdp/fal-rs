#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProcessedOutput {
        /// The processed images
pub images: Vec<Image>
    }
    

                /// Post Processing
/// 
/// Post Processing is an endpoint that can enhance images using a variety of techniques including grain, blur, sharpen, and more.
/// 
/// Category: image-to-image
                pub fn post_processing(params: ImageProcessingInput) -> FalRequest<ImageProcessingInput, ProcessedOutput> {
                    FalRequest::new(
                        "fal-ai/post-processing",
                        params
                    )
                }
                