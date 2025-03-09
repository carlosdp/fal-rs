#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct WanT2VResponse {
        /// The seed used for generation.
pub seed: i64,
/// The generated video file./// The generated video file./// {"url":"https://storage.googleapis.com/falserverless/web-examples/wan/t2v.mp4"}

pub video: File
    }
    

                /// Wan-2.1 Text-to-Video
/// 
/// Wan-2.1 is a text-to-video model that generates high-quality videos with high visual quality and motion diversity from text prompts
/// 
/// Category: text-to-video
/// Machine Type: H100
/// 
/// 
/// Generate a video from a prompt.
                pub fn wan_t2v(params: WanT2VRequest) -> FalRequest<WanT2VRequest, WanT2VResponse> {
                    FalRequest::new(
                        "fal-ai/wan-t2v",
                        params
                    )
                }
                