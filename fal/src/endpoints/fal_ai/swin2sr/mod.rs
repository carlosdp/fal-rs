#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SwinSrOutput {
        /// The generated image file info.
pub image: Image
    }
    

                /// SWIN2SR
/// 
/// Enhance low-resolution images with the superior quality of Swin2SR for sharper, clearer results.
/// 
/// Category: image-to-image
/// Machine Type: A100
                pub fn swin2sr(params: SwinSrInput) -> FalRequest<SwinSrInput, SwinSrOutput> {
                    FalRequest::new(
                        "fal-ai/swin2sr",
                        params
                    )
                }
                