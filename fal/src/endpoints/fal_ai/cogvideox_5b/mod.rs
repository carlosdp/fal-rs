#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod image_to_video;
pub mod video_to_video;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// The prompt used for generating the video.
pub prompt: String,
/// Seed of the generated video. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings,
/// The URL to the generated video
pub video: File
    }
    

                /// CogVideoX-5B
/// 
/// Generate videos from prompts using CogVideoX-5B
/// 
/// Category: text-to-video
/// Machine Type: H100
/// 
/// 
/// CogVideoX-5B, next generation text-to-video model.
                pub fn cogvideox_5b(params: BaseInput) -> FalRequest<BaseInput, Output> {
                    FalRequest::new(
                        "fal-ai/cogvideox-5b",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// The prompt used for generating the video.
pub prompt: String,
/// Seed of the generated video. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings,
/// The URL to the generated video
pub video: File
    }
    

                /// CogVideoX-5B
/// 
/// Generate videos from videos and prompts using CogVideoX-5B
/// 
/// Category: video-to-video
/// Machine Type: H100
/// 
/// 
/// CogVideoX-5B, next generation text-to-video model.
                pub fn cogvideox_5b(params: BaseInput) -> FalRequest<BaseInput, Output> {
                    FalRequest::new(
                        "fal-ai/cogvideox-5b/video-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// The prompt used for generating the video.
pub prompt: String,
/// Seed of the generated video. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings,
/// The URL to the generated video
pub video: File
    }
    

                /// CogVideoX-5B
/// 
/// Generate videos from images and prompts using CogVideoX-5B
/// 
/// Category: image-to-video
/// Machine Type: H100
/// 
/// 
/// CogVideoX-5B, next generation text-to-video model.
                pub fn cogvideox_5b(params: BaseInput) -> FalRequest<BaseInput, Output> {
                    FalRequest::new(
                        "fal-ai/cogvideox-5b/image-to-video",
                        params
                    )
                }
                