#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ControlNeXtOutput {
        /// The generated video.
pub video: File
    }
    

                /// ControlNeXt SVD
/// 
/// Animate a reference image with a driving video using ControlNeXt.
/// 
/// Category: video-to-video
/// Machine Type: A100
/// License Type: commercial
                pub fn controlnext(params: ControlNeXtInput) -> FalRequest<ControlNeXtInput, ControlNeXtOutput> {
                    FalRequest::new(
                        "fal-ai/controlnext",
                        params
                    )
                }
                