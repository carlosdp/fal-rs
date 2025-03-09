#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ray2I2VOutput {
        /// URL of the generated video/// URL of the generated video/// {"url":"https://v3.fal.media/files/zebra/9aDde3Te2kuJYHdR0Kz8R_output.mp4"}

pub video: File
    }
    

                /// Luma Ray 2
/// 
/// Ray2 is a large-scale video generative model capable of creating realistic visuals with natural, coherent motion.
/// 
/// Category: text-to-video
/// Machine Type: A100
/// 
/// 
/// Luma's state of the art Ray2 model for image-to-video generation.
/// 
/// Takes initial and/or final images and generates a video that starts from
/// and/or ends with those images.
                pub fn image_to_video(params: Ray2ImageToVideoRequest) -> FalRequest<Ray2ImageToVideoRequest, Ray2I2VOutput> {
                    FalRequest::new(
                        "fal-ai/luma-dream-machine/ray-2",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ray2I2VOutput {
        /// URL of the generated video/// URL of the generated video/// {"url":"https://v3.fal.media/files/zebra/9aDde3Te2kuJYHdR0Kz8R_output.mp4"}

pub video: File
    }
    

                /// Luma Ray 2 (Image to Video)
/// 
/// Ray2 is a large-scale video generative model capable of creating realistic visuals with natural, coherent motion.
/// 
/// Category: image-to-video
/// Machine Type: A100
/// 
/// 
/// Luma's state of the art Ray2 model for image-to-video generation.
/// 
/// Takes initial and/or final images and generates a video that starts from
/// and/or ends with those images.
                pub fn image_to_video(params: Ray2ImageToVideoRequest) -> FalRequest<Ray2ImageToVideoRequest, Ray2I2VOutput> {
                    FalRequest::new(
                        "fal-ai/luma-dream-machine/ray-2/image-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ray2I2VOutput {
        /// URL of the generated video/// URL of the generated video/// {"url":"https://v3.fal.media/files/zebra/9aDde3Te2kuJYHdR0Kz8R_output.mp4"}

pub video: File
    }
    

                /// Luma Dream Machine
/// 
/// Generate video clips from your prompts using Luma Dream Machine v1.5
/// 
/// Category: text-to-video
/// Machine Type: A100
/// 
/// 
/// Luma's state of the art Ray2 model for image-to-video generation.
/// 
/// Takes initial and/or final images and generates a video that starts from
/// and/or ends with those images.
                pub fn image_to_video(params: Ray2ImageToVideoRequest) -> FalRequest<Ray2ImageToVideoRequest, Ray2I2VOutput> {
                    FalRequest::new(
                        "fal-ai/luma-dream-machine",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ray2I2VOutput {
        /// URL of the generated video/// URL of the generated video/// {"url":"https://v3.fal.media/files/zebra/9aDde3Te2kuJYHdR0Kz8R_output.mp4"}

pub video: File
    }
    

                /// Luma Dream Machine
/// 
/// Generate video clips from your images using Luma Dream Machine v1.5
/// 
/// Category: image-to-video
/// Machine Type: A100
/// 
/// 
/// Luma's state of the art Ray2 model for image-to-video generation.
/// 
/// Takes initial and/or final images and generates a video that starts from
/// and/or ends with those images.
                pub fn image_to_video(params: Ray2ImageToVideoRequest) -> FalRequest<Ray2ImageToVideoRequest, Ray2I2VOutput> {
                    FalRequest::new(
                        "fal-ai/luma-dream-machine/image-to-video",
                        params
                    )
                }
                