#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// The stitched video/// The stitched video/// {"content_type":"video/mp4","url":"https://storage.googleapis.com/falserverless/videos/h0jgPaO6AJAbyrsNYNbGl_upscaled_video.mp4"}

pub video: File
    }
    

                /// Video Upscaler
/// 
/// The video upscaler endpoint uses RealESRGAN on each frame of the input video to upscale the video to a higher resolution.
/// 
/// Category: video-to-video
/// Machine Type: A6000
                pub fn video_upscaler(params: Input) -> FalRequest<Input, Output> {
                    FalRequest::new(
                        "fal-ai/video-upscaler",
                        params
                    )
                }
                