#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod v2;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Image with background removed
pub image: Image,
/// Mask used to remove the background
#[serde(skip_serializing_if = "Option::is_none")]
pub mask_image: Option<Option<Image>>
    }
    

                /// Birefnet Background Removal
/// 
/// bilateral reference framework (BiRefNet) for high-resolution dichotomous image segmentation (DIS)
/// 
/// Category: image-to-image
/// Machine Type: A100
                pub fn birefnet(params: Input) -> FalRequest<Input, Output> {
                    FalRequest::new(
                        "fal-ai/birefnet/v2",
                        params
                    )
                }
                