#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HunyuanT2VResponse {
        /// The seed used for generating the video.
pub seed: i64,
pub video: File
    }
    

                /// Hunyuan Video
/// 
/// Hunyuan Video is an Open video generation model with high visual quality, motion diversity, text-video alignment, and generation stability. This endpoint generates videos from text descriptions.
/// 
/// Category: text-to-video
/// Machine Type: H100
/// 
/// 
/// Hunyuan Video API for fast video generation. Text-to-video and video-to-video modes are supported.
                pub fn video_to_video(params: HunyuanV2VRequest) -> FalRequest<HunyuanV2VRequest, HunyuanT2VResponse> {
                    FalRequest::new(
                        "fal-ai/hunyuan-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HunyuanT2VResponse {
        /// The seed used for generating the video.
pub seed: i64,
pub video: File
    }
    

                /// Hunyuan Video (Video-to-Video)
/// 
/// Hunyuan Video is an Open video generation model with high visual quality, motion diversity, text-video alignment, and generation stability. Use this endpoint to generate videos from videos.
/// 
/// Category: video-to-video
/// Machine Type: H100
/// 
/// 
/// Hunyuan Video API for fast video generation. Text-to-video and video-to-video modes are supported.
                pub fn video_to_video(params: HunyuanV2VRequest) -> FalRequest<HunyuanV2VRequest, HunyuanT2VResponse> {
                    FalRequest::new(
                        "fal-ai/hunyuan-video/video-to-video",
                        params
                    )
                }
                