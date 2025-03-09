#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// The generated video with the lip sync.
pub video: File
    }
    

                /// LatentSync
/// 
/// LatentSync is a video-to-video model that generates lip sync animations from audio using advanced algorithms for high-quality synchronization.
/// 
/// Category: video-to-video
/// 
/// License Type: commercial
                pub fn latentsync(params: Input) -> FalRequest<Input, Output> {
                    FalRequest::new(
                        "fal-ai/latentsync",
                        params
                    )
                }
                