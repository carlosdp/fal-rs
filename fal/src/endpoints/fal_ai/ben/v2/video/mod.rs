#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ben2OutputVideo {
        /// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
/// The generated video file./// The generated video file./// {"content_type":"video/mp4","url":"https://storage.googleapis.com/falserverless/gallery/Ben2/foreground.mp4"}

pub video: File
    }
    

                /// ben-v2-image
/// 
/// A fast and high quality model for image background removal.
/// 
/// Category: image-to-image
/// Machine Type: A100
                pub fn video(params: Ben2InputVideo) -> FalRequest<Ben2InputVideo, Ben2OutputVideo> {
                    FalRequest::new(
                        "fal-ai/ben/v2/image",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ben2OutputVideo {
        /// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
/// The generated video file./// The generated video file./// {"content_type":"video/mp4","url":"https://storage.googleapis.com/falserverless/gallery/Ben2/foreground.mp4"}

pub video: File
    }
    

                /// Ben-Video-Bg-Rm
/// 
/// A model for high quality and smooth background removal for videos.
/// 
/// Category: video-to-video
/// Machine Type: A100
                pub fn video(params: Ben2InputVideo) -> FalRequest<Ben2InputVideo, Ben2OutputVideo> {
                    FalRequest::new(
                        "fal-ai/ben/v2/video",
                        params
                    )
                }
                