#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OmniZeroOutput {
        /// The generated image./// The generated image./// {"content_type":"image/png","height":1024,"url":"https://storage.googleapis.com/falserverless/model_tests/omni_zero/result.png","width":1024}

pub image: Image
    }
    

                /// Omni Zero
/// 
/// Any pose, any style, any identity
/// 
/// Category: image-to-image
/// Machine Type: A100
                pub fn omni_zero(params: OmniZeroInput) -> FalRequest<OmniZeroInput, OmniZeroOutput> {
                    FalRequest::new(
                        "fal-ai/omni-zero",
                        params
                    )
                }
                