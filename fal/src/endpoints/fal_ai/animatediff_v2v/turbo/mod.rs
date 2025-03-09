#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AnimateDiffV2VTurboOutput {
        /// Seed used for generating the video.
pub seed: i64,
pub timings: Timings,
/// Generated video file./// Generated video file./// {"content_type":"video/mp4","url":"https://storage.googleapis.com/falserverless/model_tests/animatediff_v2v/rocket-output.mp4"}

pub video: File
    }
    

                /// AnimateDiff Video-to-Video Evolved
/// 
/// Re-animate your videos with evolved consistency!
/// 
/// Category: video-to-video
/// Machine Type: A100
                pub fn turbo(params: AnimateDiffV2VTurboInput) -> FalRequest<AnimateDiffV2VTurboInput, AnimateDiffV2VTurboOutput> {
                    FalRequest::new(
                        "fal-ai/animatediff-v2v",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AnimateDiffV2VTurboOutput {
        /// Seed used for generating the video.
pub seed: i64,
pub timings: Timings,
/// Generated video file./// Generated video file./// {"content_type":"video/mp4","url":"https://storage.googleapis.com/falserverless/model_tests/animatediff_v2v/rocket-output.mp4"}

pub video: File
    }
    

                /// AnimateDiff Video-to-Video Evolved
/// 
/// Re-animate your videos with evolved consistency!
/// 
/// Category: video-to-video
/// Machine Type: A100
                pub fn turbo(params: AnimateDiffV2VTurboInput) -> FalRequest<AnimateDiffV2VTurboInput, AnimateDiffV2VTurboOutput> {
                    FalRequest::new(
                        "fal-ai/animatediff-v2v/turbo",
                        params
                    )
                }
                