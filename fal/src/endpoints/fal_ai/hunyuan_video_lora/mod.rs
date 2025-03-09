#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HunyuanT2VResponse {
        /// The seed used for generating the video.
pub seed: i64,
pub video: File
    }
    

                /// Hunyuan Video LoRA Inference
/// 
/// Hunyuan Video is an Open video generation model with high visual quality, motion diversity, text-video alignment, and generation stability
/// 
/// Category: text-to-video
/// Machine Type: H100
                pub fn hunyuan_video_lora(params: HunyuanT2VRequest) -> FalRequest<HunyuanT2VRequest, HunyuanT2VResponse> {
                    FalRequest::new(
                        "fal-ai/hunyuan-video-lora",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HunyuanT2VResponse {
        /// The seed used for generating the video.
pub seed: i64,
pub video: File
    }
    

                /// Hunyuan Video LoRA Inference (Video-to-Video)
/// 
/// Hunyuan Video is an Open video generation model with high visual quality, motion diversity, text-video alignment, and generation stability. Use this endpoint to generate videos from videos.
/// 
/// Category: video-to-video
/// Machine Type: H100
                pub fn hunyuan_video_lora(params: HunyuanT2VRequest) -> FalRequest<HunyuanT2VRequest, HunyuanT2VResponse> {
                    FalRequest::new(
                        "fal-ai/hunyuan-video-lora/video-to-video",
                        params
                    )
                }
                