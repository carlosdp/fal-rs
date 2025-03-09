#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AnimateDiffT2VOutput {
        /// Seed used for generating the video.
pub seed: i64,
/// Generated video file./// Generated video file./// {"url":"https://fal-cdn.batuhan-941.workers.dev/files/kangaroo/DSrFBOk9XXIplm_kukI4n.mp4"}

pub video: File
    }
    

                /// AnimateDiff
/// 
/// Animate your ideas!
/// 
/// Category: text-to-video
/// Machine Type: A100
                pub fn text_to_video(params: AnimateDiffT2VTurboInput) -> FalRequest<AnimateDiffT2VTurboInput, AnimateDiffT2VOutput> {
                    FalRequest::new(
                        "fal-ai/fast-animatediff/text-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AnimateDiffT2VOutput {
        /// Seed used for generating the video.
pub seed: i64,
/// Generated video file./// Generated video file./// {"url":"https://fal-cdn.batuhan-941.workers.dev/files/kangaroo/DSrFBOk9XXIplm_kukI4n.mp4"}

pub video: File
    }
    

                /// AnimateDiff
/// 
/// Re-animate your videos!
/// 
/// Category: video-to-video
/// Machine Type: A100
                pub fn text_to_video(params: AnimateDiffT2VTurboInput) -> FalRequest<AnimateDiffT2VTurboInput, AnimateDiffT2VOutput> {
                    FalRequest::new(
                        "fal-ai/fast-animatediff/video-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AnimateDiffT2VOutput {
        /// Seed used for generating the video.
pub seed: i64,
/// Generated video file./// Generated video file./// {"url":"https://fal-cdn.batuhan-941.workers.dev/files/kangaroo/DSrFBOk9XXIplm_kukI4n.mp4"}

pub video: File
    }
    

                /// AnimateDiff Turbo
/// 
/// Animate your ideas in lightning speed!
/// 
/// Category: text-to-video
/// Machine Type: A100
                pub fn text_to_video(params: AnimateDiffT2VTurboInput) -> FalRequest<AnimateDiffT2VTurboInput, AnimateDiffT2VOutput> {
                    FalRequest::new(
                        "fal-ai/fast-animatediff/turbo/text-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AnimateDiffT2VOutput {
        /// Seed used for generating the video.
pub seed: i64,
/// Generated video file./// Generated video file./// {"url":"https://fal-cdn.batuhan-941.workers.dev/files/kangaroo/DSrFBOk9XXIplm_kukI4n.mp4"}

pub video: File
    }
    

                /// AnimateDiff Turbo
/// 
/// Re-animate your videos in lightning speed!
/// 
/// Category: video-to-video
/// Machine Type: A100
                pub fn text_to_video(params: AnimateDiffT2VTurboInput) -> FalRequest<AnimateDiffT2VTurboInput, AnimateDiffT2VOutput> {
                    FalRequest::new(
                        "fal-ai/fast-animatediff/turbo/video-to-video",
                        params
                    )
                }
                