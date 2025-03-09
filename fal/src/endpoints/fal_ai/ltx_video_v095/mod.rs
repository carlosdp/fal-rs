#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod extend;
pub mod image_to_video;
pub mod multiconditioning;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TextToVideoOutput {
        /// The seed used for generation.
pub seed: i64,
/// The generated video file./// The generated video file./// {"url":"https://storage.googleapis.com/falserverless/example_outputs/ltx-t2v_output.mp4"}

pub video: File
    }
    

                /// LTX Video-0.9.5
/// 
/// Generate videos from prompts using LTX Video-0.9.5
/// 
/// Category: text-to-video
/// Machine Type: H100
/// 
/// 
/// Generate a video from a prompt.
                pub fn ltx_video_v095(params: TextToVideoInput) -> FalRequest<TextToVideoInput, TextToVideoOutput> {
                    FalRequest::new(
                        "fal-ai/ltx-video-v095",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TextToVideoOutput {
        /// The seed used for generation.
pub seed: i64,
/// The generated video file./// The generated video file./// {"url":"https://storage.googleapis.com/falserverless/example_outputs/ltx-t2v_output.mp4"}

pub video: File
    }
    

                /// LTX Video-0.9.5
/// 
/// Generate videos from prompts and images using LTX Video-0.9.5
/// 
/// Category: image-to-video
/// Machine Type: H100
/// 
/// 
/// Generate a video from a prompt.
                pub fn ltx_video_v095(params: TextToVideoInput) -> FalRequest<TextToVideoInput, TextToVideoOutput> {
                    FalRequest::new(
                        "fal-ai/ltx-video-v095/image-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TextToVideoOutput {
        /// The seed used for generation.
pub seed: i64,
/// The generated video file./// The generated video file./// {"url":"https://storage.googleapis.com/falserverless/example_outputs/ltx-t2v_output.mp4"}

pub video: File
    }
    

                /// LTX Video-0.9.5
/// 
/// Generate videos from prompts and videos using LTX Video-0.9.5
/// 
/// Category: video-to-video
/// Machine Type: H100
/// 
/// 
/// Generate a video from a prompt.
                pub fn ltx_video_v095(params: TextToVideoInput) -> FalRequest<TextToVideoInput, TextToVideoOutput> {
                    FalRequest::new(
                        "fal-ai/ltx-video-v095/extend",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TextToVideoOutput {
        /// The seed used for generation.
pub seed: i64,
/// The generated video file./// The generated video file./// {"url":"https://storage.googleapis.com/falserverless/example_outputs/ltx-t2v_output.mp4"}

pub video: File
    }
    

                /// LTX Video-0.9.5
/// 
/// Generate videos from prompts,images, and videos using LTX Video-0.9.5
/// 
/// Category: video-to-video
/// Machine Type: H100
/// 
/// 
/// Generate a video from a prompt.
                pub fn ltx_video_v095(params: TextToVideoInput) -> FalRequest<TextToVideoInput, TextToVideoOutput> {
                    FalRequest::new(
                        "fal-ai/ltx-video-v095/multiconditioning",
                        params
                    )
                }
                