#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ImageOutput {
        /// The segmented output image
pub image: File
    }
    

                /// EVF-SAM2 Segmentation
/// 
/// EVF-SAM2 combines natural language understanding with advanced segmentation capabilities, allowing you to precisely mask image regions using intuitive positive and negative text prompts.
/// 
/// Category: image-to-image
                pub fn evf_sam(params: ImageInput) -> FalRequest<ImageInput, ImageOutput> {
                    FalRequest::new(
                        "fal-ai/evf-sam",
                        params
                    )
                }
                