#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod image_to_video;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VideoOutput {
        /// The generated video/// The generated video/// {"url":"https://fal.media/files/monkey/vNZqQV_WgC9MhoidClLyw_output.mp4"}

pub video: File
    }
    

                /// MiniMax (Hailuo AI) Video 01 Live
/// 
/// Generate video clips from your prompts using MiniMax model
/// 
/// Category: text-to-video
/// 
/// 
/// 
/// Hailuo T2V-01 API: Native high-resolution, high-frame-rate video generation model, supports text-to-video and image-to-video
                pub fn video_01(params: TextToVideoRequest) -> FalRequest<TextToVideoRequest, VideoOutput> {
                    FalRequest::new(
                        "fal-ai/minimax/video-01-live",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VideoOutput {
        /// The generated video/// The generated video/// {"url":"https://fal.media/files/monkey/vNZqQV_WgC9MhoidClLyw_output.mp4"}

pub video: File
    }
    

                /// MiniMax (Hailuo AI) Video 01 Live
/// 
/// Generate video clips from your images using MiniMax Video model
/// 
/// Category: image-to-video
/// 
/// 
/// 
/// Hailuo T2V-01 API: Native high-resolution, high-frame-rate video generation model, supports text-to-video and image-to-video
                pub fn video_01(params: TextToVideoRequest) -> FalRequest<TextToVideoRequest, VideoOutput> {
                    FalRequest::new(
                        "fal-ai/minimax/video-01-live/image-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VideoOutput {
        /// The generated video/// The generated video/// {"url":"https://fal.media/files/monkey/vNZqQV_WgC9MhoidClLyw_output.mp4"}

pub video: File
    }
    

                /// MiniMax (Hailuo AI) Video 01 Subject Reference
/// 
/// Generate video clips maintaining consistent, realistic facial features and identity across dynamic video content
/// 
/// Category: image-to-video
/// 
/// 
/// 
/// Hailuo T2V-01 API: Native high-resolution, high-frame-rate video generation model, supports text-to-video and image-to-video
                pub fn video_01(params: TextToVideoRequest) -> FalRequest<TextToVideoRequest, VideoOutput> {
                    FalRequest::new(
                        "fal-ai/minimax/video-01-subject-reference",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VideoOutput {
        /// The generated video/// The generated video/// {"url":"https://fal.media/files/monkey/vNZqQV_WgC9MhoidClLyw_output.mp4"}

pub video: File
    }
    

                /// MiniMax (Hailuo AI) Video 01 Director
/// 
/// Generate video clips more accurately with respect to natural language descriptions and using camera movement instructions for shot control.
/// 
/// Category: text-to-video
/// 
/// 
/// 
/// Hailuo T2V-01 API: Native high-resolution, high-frame-rate video generation model, supports text-to-video and image-to-video
                pub fn video_01(params: TextToVideoRequest) -> FalRequest<TextToVideoRequest, VideoOutput> {
                    FalRequest::new(
                        "fal-ai/minimax/video-01-director",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VideoOutput {
        /// The generated video/// The generated video/// {"url":"https://fal.media/files/monkey/vNZqQV_WgC9MhoidClLyw_output.mp4"}

pub video: File
    }
    

                /// MiniMax (Hailuo AI) Video 01 Director - Image to Video
/// 
/// Generate video clips more accurately with respect to initial image, natural language descriptions, and using camera movement instructions for shot control.
/// 
/// Category: image-to-video
/// 
/// 
/// 
/// Hailuo T2V-01 API: Native high-resolution, high-frame-rate video generation model, supports text-to-video and image-to-video
                pub fn video_01(params: TextToVideoRequest) -> FalRequest<TextToVideoRequest, VideoOutput> {
                    FalRequest::new(
                        "fal-ai/minimax/video-01-director/image-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VideoOutput {
        /// The generated video/// The generated video/// {"url":"https://fal.media/files/monkey/vNZqQV_WgC9MhoidClLyw_output.mp4"}

pub video: File
    }
    

                /// MiniMax (Hailuo AI) Video 01
/// 
/// Generate video clips from your images using MiniMax Video model
/// 
/// Category: image-to-video
/// Machine Type: A100
/// 
/// 
/// Hailuo T2V-01 API: Native high-resolution, high-frame-rate video generation model, supports text-to-video and image-to-video
                pub fn video_01(params: TextToVideoRequest) -> FalRequest<TextToVideoRequest, VideoOutput> {
                    FalRequest::new(
                        "fal-ai/minimax/video-01/image-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VideoOutput {
        /// The generated video/// The generated video/// {"url":"https://fal.media/files/monkey/vNZqQV_WgC9MhoidClLyw_output.mp4"}

pub video: File
    }
    

                /// MiniMax (Hailuo AI) Video 01
/// 
/// Generate video clips from your prompts using MiniMax model
/// 
/// Category: text-to-video
/// Machine Type: A100
/// 
/// 
/// Hailuo T2V-01 API: Native high-resolution, high-frame-rate video generation model, supports text-to-video and image-to-video
                pub fn video_01(params: TextToVideoRequest) -> FalRequest<TextToVideoRequest, VideoOutput> {
                    FalRequest::new(
                        "fal-ai/minimax/video-01",
                        params
                    )
                }
                