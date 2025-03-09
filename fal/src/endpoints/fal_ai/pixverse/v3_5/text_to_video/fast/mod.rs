#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VideoOutput {
        /// The generated video/// The generated video/// {"content_type":"video/mp4","file_name":"output.mp4","file_size":2995630,"url":"https://fal.media/files/zebra/11UahivZ3XZ1tRlcEcgPq_output.mp4"}

pub video: File
    }
    

                /// PixVerse v3.5
/// 
/// Generate high quality video clips from text prompts using PixVerse v3.5
/// 
/// Category: text-to-video
/// Machine Type: A100
                pub fn fast(params: FastTextToVideoRequest) -> FalRequest<FastTextToVideoRequest, VideoOutput> {
                    FalRequest::new(
                        "fal-ai/pixverse/v3.5/text-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VideoOutput {
        /// The generated video/// The generated video/// {"content_type":"video/mp4","file_name":"output.mp4","file_size":2995630,"url":"https://fal.media/files/zebra/11UahivZ3XZ1tRlcEcgPq_output.mp4"}

pub video: File
    }
    

                /// PixVerse v3.5: Image to Video
/// 
/// Generate high quality video clips from text and image prompts using PixVerse v3.5
/// 
/// Category: image-to-video
/// Machine Type: A100
                pub fn fast(params: FastTextToVideoRequest) -> FalRequest<FastTextToVideoRequest, VideoOutput> {
                    FalRequest::new(
                        "fal-ai/pixverse/v3.5/image-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VideoOutput {
        /// The generated video/// The generated video/// {"content_type":"video/mp4","file_name":"output.mp4","file_size":2995630,"url":"https://fal.media/files/zebra/11UahivZ3XZ1tRlcEcgPq_output.mp4"}

pub video: File
    }
    

                /// PixVerse v3.5 Fast
/// 
/// Generate high quality video clips quickly from text prompts using PixVerse v3.5 Fast
/// 
/// Category: text-to-video
/// Machine Type: A100
                pub fn fast(params: FastTextToVideoRequest) -> FalRequest<FastTextToVideoRequest, VideoOutput> {
                    FalRequest::new(
                        "fal-ai/pixverse/v3.5/text-to-video/fast",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VideoOutput {
        /// The generated video/// The generated video/// {"content_type":"video/mp4","file_name":"output.mp4","file_size":2995630,"url":"https://fal.media/files/zebra/11UahivZ3XZ1tRlcEcgPq_output.mp4"}

pub video: File
    }
    

                /// PixVerse v3.5: Image to Video Fast
/// 
/// Generate high quality video clips from text and image prompts quickly using PixVerse v3.5 Fast
/// 
/// Category: image-to-video
/// Machine Type: A100
                pub fn fast(params: FastTextToVideoRequest) -> FalRequest<FastTextToVideoRequest, VideoOutput> {
                    FalRequest::new(
                        "fal-ai/pixverse/v3.5/image-to-video/fast",
                        params
                    )
                }
                