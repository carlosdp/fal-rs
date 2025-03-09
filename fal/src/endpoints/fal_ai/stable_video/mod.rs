#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod text_to_video;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VideoOutput {
        /// Seed for random number generator
pub seed: i64,
/// Generated video
pub video: File
    }
    

                /// High Quality Stable Video Diffusion
/// 
/// Generate short video clips from your images using SVD v1.1
/// 
/// Category: image-to-video
/// Machine Type: A100
                pub fn stable_video(params: ImageInput) -> FalRequest<ImageInput, VideoOutput> {
                    FalRequest::new(
                        "fal-ai/stable-video",
                        params
                    )
                }
                