#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct I2VOutput {
        /// The generated video/// The generated video/// {"content_type":"video/mp4","file_name":"output.mp4","file_size":4060052,"url":"https://fal.media/files/tiger/8V9H8RLyFiWjmJDOxGbcG_output.mp4"}

pub video: File
    }
    

                /// PixVerse v3.5
/// 
/// Generate high quality video clips from text prompts using PixVerse v3.5
/// 
/// Category: text-to-video
/// Machine Type: A100
                pub fn fast(params: FastImageToVideoRequest) -> FalRequest<FastImageToVideoRequest, I2VOutput> {
                    FalRequest::new(
                        "fal-ai/pixverse/v3.5/text-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct I2VOutput {
        /// The generated video/// The generated video/// {"content_type":"video/mp4","file_name":"output.mp4","file_size":4060052,"url":"https://fal.media/files/tiger/8V9H8RLyFiWjmJDOxGbcG_output.mp4"}

pub video: File
    }
    

                /// PixVerse v3.5: Image to Video
/// 
/// Generate high quality video clips from text and image prompts using PixVerse v3.5
/// 
/// Category: image-to-video
/// Machine Type: A100
                pub fn fast(params: FastImageToVideoRequest) -> FalRequest<FastImageToVideoRequest, I2VOutput> {
                    FalRequest::new(
                        "fal-ai/pixverse/v3.5/image-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct I2VOutput {
        /// The generated video/// The generated video/// {"content_type":"video/mp4","file_name":"output.mp4","file_size":4060052,"url":"https://fal.media/files/tiger/8V9H8RLyFiWjmJDOxGbcG_output.mp4"}

pub video: File
    }
    

                /// PixVerse v3.5 Fast
/// 
/// Generate high quality video clips quickly from text prompts using PixVerse v3.5 Fast
/// 
/// Category: text-to-video
/// Machine Type: A100
                pub fn fast(params: FastImageToVideoRequest) -> FalRequest<FastImageToVideoRequest, I2VOutput> {
                    FalRequest::new(
                        "fal-ai/pixverse/v3.5/text-to-video/fast",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct I2VOutput {
        /// The generated video/// The generated video/// {"content_type":"video/mp4","file_name":"output.mp4","file_size":4060052,"url":"https://fal.media/files/tiger/8V9H8RLyFiWjmJDOxGbcG_output.mp4"}

pub video: File
    }
    

                /// PixVerse v3.5: Image to Video Fast
/// 
/// Generate high quality video clips from text and image prompts quickly using PixVerse v3.5 Fast
/// 
/// Category: image-to-video
/// Machine Type: A100
                pub fn fast(params: FastImageToVideoRequest) -> FalRequest<FastImageToVideoRequest, I2VOutput> {
                    FalRequest::new(
                        "fal-ai/pixverse/v3.5/image-to-video/fast",
                        params
                    )
                }
                