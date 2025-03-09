#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct I2VOutput {
        /// The generated video/// The generated video/// {"url":"https://v2.fal.media/files/8c216fcbc4ed41cd8840bd48c1ec6dd6_output.mp4"}

pub video: File
    }
    

                /// Luma Ray 2
/// 
/// Ray2 is a large-scale video generative model capable of creating realistic visuals with natural, coherent motion.
/// 
/// Category: text-to-video
/// Machine Type: A100
                pub fn image_to_video(params: ImageToVideoRequest) -> FalRequest<ImageToVideoRequest, I2VOutput> {
                    FalRequest::new(
                        "fal-ai/luma-dream-machine/ray-2",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct I2VOutput {
        /// The generated video/// The generated video/// {"url":"https://v2.fal.media/files/8c216fcbc4ed41cd8840bd48c1ec6dd6_output.mp4"}

pub video: File
    }
    

                /// Luma Ray 2 (Image to Video)
/// 
/// Ray2 is a large-scale video generative model capable of creating realistic visuals with natural, coherent motion.
/// 
/// Category: image-to-video
/// Machine Type: A100
                pub fn image_to_video(params: ImageToVideoRequest) -> FalRequest<ImageToVideoRequest, I2VOutput> {
                    FalRequest::new(
                        "fal-ai/luma-dream-machine/ray-2/image-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct I2VOutput {
        /// The generated video/// The generated video/// {"url":"https://v2.fal.media/files/8c216fcbc4ed41cd8840bd48c1ec6dd6_output.mp4"}

pub video: File
    }
    

                /// Luma Dream Machine
/// 
/// Generate video clips from your prompts using Luma Dream Machine v1.5
/// 
/// Category: text-to-video
/// Machine Type: A100
                pub fn image_to_video(params: ImageToVideoRequest) -> FalRequest<ImageToVideoRequest, I2VOutput> {
                    FalRequest::new(
                        "fal-ai/luma-dream-machine",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct I2VOutput {
        /// The generated video/// The generated video/// {"url":"https://v2.fal.media/files/8c216fcbc4ed41cd8840bd48c1ec6dd6_output.mp4"}

pub video: File
    }
    

                /// Luma Dream Machine
/// 
/// Generate video clips from your images using Luma Dream Machine v1.5
/// 
/// Category: image-to-video
/// Machine Type: A100
                pub fn image_to_video(params: ImageToVideoRequest) -> FalRequest<ImageToVideoRequest, I2VOutput> {
                    FalRequest::new(
                        "fal-ai/luma-dream-machine/image-to-video",
                        params
                    )
                }
                