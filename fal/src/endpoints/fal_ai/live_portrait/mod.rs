#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod image;
pub mod video;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LivePortraitOutput {
        /// The generated video file.
pub video: File
    }
    

                /// Live Portrait
/// 
/// Transfer expression from a video to a portrait.
/// 
/// Category: image-to-video
/// Machine Type: A6000
/// License Type: commercial
                pub fn live_portrait(params: LivePortraitInput) -> FalRequest<LivePortraitInput, LivePortraitOutput> {
                    FalRequest::new(
                        "fal-ai/live-portrait",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LivePortraitOutput {
        /// The generated video file.
pub video: File
    }
    

                /// Live Portrait
/// 
/// Transfer expression from a video to a portrait.
/// 
/// Category: image-to-image
/// Machine Type: A6000
/// License Type: commercial
                pub fn live_portrait(params: LivePortraitInput) -> FalRequest<LivePortraitInput, LivePortraitOutput> {
                    FalRequest::new(
                        "fal-ai/live-portrait/image",
                        params
                    )
                }
                