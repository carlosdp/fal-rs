#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AnimateDiffV2VOutput {
        /// Seed used for generating the video.
pub seed: i64,
/// Generated video file./// Generated video file./// {"url":"https://fal-cdn.batuhan-941.workers.dev/files/koala/5Cb_6P_s9wW8f8-g9c4yj.mp4"}

pub video: File
    }
    

                /// AnimateDiff
/// 
/// Animate your ideas!
/// 
/// Category: text-to-video
/// Machine Type: A100
                pub fn video_to_video(params: AnimateDiffV2VTurboInput) -> FalRequest<AnimateDiffV2VTurboInput, AnimateDiffV2VOutput> {
                    FalRequest::new(
                        "fal-ai/fast-animatediff/text-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AnimateDiffV2VOutput {
        /// Seed used for generating the video.
pub seed: i64,
/// Generated video file./// Generated video file./// {"url":"https://fal-cdn.batuhan-941.workers.dev/files/koala/5Cb_6P_s9wW8f8-g9c4yj.mp4"}

pub video: File
    }
    

                /// AnimateDiff
/// 
/// Re-animate your videos!
/// 
/// Category: video-to-video
/// Machine Type: A100
                pub fn video_to_video(params: AnimateDiffV2VTurboInput) -> FalRequest<AnimateDiffV2VTurboInput, AnimateDiffV2VOutput> {
                    FalRequest::new(
                        "fal-ai/fast-animatediff/video-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AnimateDiffV2VOutput {
        /// Seed used for generating the video.
pub seed: i64,
/// Generated video file./// Generated video file./// {"url":"https://fal-cdn.batuhan-941.workers.dev/files/koala/5Cb_6P_s9wW8f8-g9c4yj.mp4"}

pub video: File
    }
    

                /// AnimateDiff Turbo
/// 
/// Animate your ideas in lightning speed!
/// 
/// Category: text-to-video
/// Machine Type: A100
                pub fn video_to_video(params: AnimateDiffV2VTurboInput) -> FalRequest<AnimateDiffV2VTurboInput, AnimateDiffV2VOutput> {
                    FalRequest::new(
                        "fal-ai/fast-animatediff/turbo/text-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AnimateDiffV2VOutput {
        /// Seed used for generating the video.
pub seed: i64,
/// Generated video file./// Generated video file./// {"url":"https://fal-cdn.batuhan-941.workers.dev/files/koala/5Cb_6P_s9wW8f8-g9c4yj.mp4"}

pub video: File
    }
    

                /// AnimateDiff Turbo
/// 
/// Re-animate your videos in lightning speed!
/// 
/// Category: video-to-video
/// Machine Type: A100
                pub fn video_to_video(params: AnimateDiffV2VTurboInput) -> FalRequest<AnimateDiffV2VTurboInput, AnimateDiffV2VOutput> {
                    FalRequest::new(
                        "fal-ai/fast-animatediff/turbo/video-to-video",
                        params
                    )
                }
                