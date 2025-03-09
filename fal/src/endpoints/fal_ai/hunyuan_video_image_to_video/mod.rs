#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HunyuanI2VResponse {
        /// The seed used for generating the video.
pub seed: i64,
pub video: File
    }
    

                /// Hunyuan Video Image-to-Video Inference
/// 
/// Image to Video for the high-quality Hunyuan Video I2V model.
/// 
/// Category: image-to-video
/// Machine Type: H100
                pub fn hunyuan_video_image_to_video(params: HunyuanVideoRequest) -> FalRequest<HunyuanVideoRequest, HunyuanI2VResponse> {
                    FalRequest::new(
                        "fal-ai/hunyuan-video-image-to-video",
                        params
                    )
                }
                