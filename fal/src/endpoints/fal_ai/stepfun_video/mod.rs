#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StepFunT2VResponse {
        /// The seed used for generating the video.
pub seed: i64,
pub video: File
    }
    

                /// Step-Video
/// 
/// Step-Video is a state-of-the-art (SoTA) text-to-video pre-trained model with 30 billion parameters and the capability to generate videos up to 204 frames.
/// 
/// Category: text-to-video
/// Machine Type: H100
                pub fn stepfun_video(params: StepFunT2VRequest) -> FalRequest<StepFunT2VRequest, StepFunT2VResponse> {
                    FalRequest::new(
                        "fal-ai/stepfun-video",
                        params
                    )
                }
                