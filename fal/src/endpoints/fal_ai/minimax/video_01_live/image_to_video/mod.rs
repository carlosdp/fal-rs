#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct I2VLiveOutput {
        /// The generated video/// The generated video/// {"url":"https://fal.media/files/monkey/bkT4T4uLOXr0jDsIMlNd5_output.mp4"}

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
/// Hailuo I2V-01-Live image-to-video API: Transform Static Art into Dynamic Masterpieces
/// 
/// Live is our latest addition to the I2V family, designed to revolutionize how 2D illustrations come to life. With
/// enhanced smoothness and vivid motion, this model lets your characters move, speak, and shine like never before. Optimized
/// for stability and subtle expression, Hailuo I2V-01-Live supports a wide range of artistic styles, empowering you to expand
/// your creative expression and bring your art to life with unparalleled fluidity and finesse.
                pub fn image_to_video(params: ImageToVideoRequest) -> FalRequest<ImageToVideoRequest, I2VLiveOutput> {
                    FalRequest::new(
                        "fal-ai/minimax/video-01-live",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct I2VLiveOutput {
        /// The generated video/// The generated video/// {"url":"https://fal.media/files/monkey/bkT4T4uLOXr0jDsIMlNd5_output.mp4"}

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
/// Hailuo I2V-01-Live image-to-video API: Transform Static Art into Dynamic Masterpieces
/// 
/// Live is our latest addition to the I2V family, designed to revolutionize how 2D illustrations come to life. With
/// enhanced smoothness and vivid motion, this model lets your characters move, speak, and shine like never before. Optimized
/// for stability and subtle expression, Hailuo I2V-01-Live supports a wide range of artistic styles, empowering you to expand
/// your creative expression and bring your art to life with unparalleled fluidity and finesse.
                pub fn image_to_video(params: ImageToVideoRequest) -> FalRequest<ImageToVideoRequest, I2VLiveOutput> {
                    FalRequest::new(
                        "fal-ai/minimax/video-01-live/image-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct I2VLiveOutput {
        /// The generated video/// The generated video/// {"url":"https://fal.media/files/monkey/bkT4T4uLOXr0jDsIMlNd5_output.mp4"}

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
/// Hailuo I2V-01-Live image-to-video API: Transform Static Art into Dynamic Masterpieces
/// 
/// Live is our latest addition to the I2V family, designed to revolutionize how 2D illustrations come to life. With
/// enhanced smoothness and vivid motion, this model lets your characters move, speak, and shine like never before. Optimized
/// for stability and subtle expression, Hailuo I2V-01-Live supports a wide range of artistic styles, empowering you to expand
/// your creative expression and bring your art to life with unparalleled fluidity and finesse.
                pub fn image_to_video(params: ImageToVideoRequest) -> FalRequest<ImageToVideoRequest, I2VLiveOutput> {
                    FalRequest::new(
                        "fal-ai/minimax/video-01-subject-reference",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct I2VLiveOutput {
        /// The generated video/// The generated video/// {"url":"https://fal.media/files/monkey/bkT4T4uLOXr0jDsIMlNd5_output.mp4"}

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
/// Hailuo I2V-01-Live image-to-video API: Transform Static Art into Dynamic Masterpieces
/// 
/// Live is our latest addition to the I2V family, designed to revolutionize how 2D illustrations come to life. With
/// enhanced smoothness and vivid motion, this model lets your characters move, speak, and shine like never before. Optimized
/// for stability and subtle expression, Hailuo I2V-01-Live supports a wide range of artistic styles, empowering you to expand
/// your creative expression and bring your art to life with unparalleled fluidity and finesse.
                pub fn image_to_video(params: ImageToVideoRequest) -> FalRequest<ImageToVideoRequest, I2VLiveOutput> {
                    FalRequest::new(
                        "fal-ai/minimax/video-01-director",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct I2VLiveOutput {
        /// The generated video/// The generated video/// {"url":"https://fal.media/files/monkey/bkT4T4uLOXr0jDsIMlNd5_output.mp4"}

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
/// Hailuo I2V-01-Live image-to-video API: Transform Static Art into Dynamic Masterpieces
/// 
/// Live is our latest addition to the I2V family, designed to revolutionize how 2D illustrations come to life. With
/// enhanced smoothness and vivid motion, this model lets your characters move, speak, and shine like never before. Optimized
/// for stability and subtle expression, Hailuo I2V-01-Live supports a wide range of artistic styles, empowering you to expand
/// your creative expression and bring your art to life with unparalleled fluidity and finesse.
                pub fn image_to_video(params: ImageToVideoRequest) -> FalRequest<ImageToVideoRequest, I2VLiveOutput> {
                    FalRequest::new(
                        "fal-ai/minimax/video-01-director/image-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct I2VLiveOutput {
        /// The generated video/// The generated video/// {"url":"https://fal.media/files/monkey/bkT4T4uLOXr0jDsIMlNd5_output.mp4"}

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
/// Hailuo I2V-01-Live image-to-video API: Transform Static Art into Dynamic Masterpieces
/// 
/// Live is our latest addition to the I2V family, designed to revolutionize how 2D illustrations come to life. With
/// enhanced smoothness and vivid motion, this model lets your characters move, speak, and shine like never before. Optimized
/// for stability and subtle expression, Hailuo I2V-01-Live supports a wide range of artistic styles, empowering you to expand
/// your creative expression and bring your art to life with unparalleled fluidity and finesse.
                pub fn image_to_video(params: ImageToVideoRequest) -> FalRequest<ImageToVideoRequest, I2VLiveOutput> {
                    FalRequest::new(
                        "fal-ai/minimax/video-01/image-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct I2VLiveOutput {
        /// The generated video/// The generated video/// {"url":"https://fal.media/files/monkey/bkT4T4uLOXr0jDsIMlNd5_output.mp4"}

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
/// Hailuo I2V-01-Live image-to-video API: Transform Static Art into Dynamic Masterpieces
/// 
/// Live is our latest addition to the I2V family, designed to revolutionize how 2D illustrations come to life. With
/// enhanced smoothness and vivid motion, this model lets your characters move, speak, and shine like never before. Optimized
/// for stability and subtle expression, Hailuo I2V-01-Live supports a wide range of artistic styles, empowering you to expand
/// your creative expression and bring your art to life with unparalleled fluidity and finesse.
                pub fn image_to_video(params: ImageToVideoRequest) -> FalRequest<ImageToVideoRequest, I2VLiveOutput> {
                    FalRequest::new(
                        "fal-ai/minimax/video-01",
                        params
                    )
                }
                