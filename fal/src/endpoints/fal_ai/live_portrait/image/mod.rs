#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LivePortraitImageOutput {
        /// The generated image file.
pub image: Image
    }
    

                /// Live Portrait
/// 
/// Transfer expression from a video to a portrait.
/// 
/// Category: image-to-video
/// Machine Type: A6000
/// License Type: commercial
                pub fn image(params: LivePortraitImageInput) -> FalRequest<LivePortraitImageInput, LivePortraitImageOutput> {
                    FalRequest::new(
                        "fal-ai/live-portrait",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LivePortraitImageOutput {
        /// The generated image file.
pub image: Image
    }
    

                /// Live Portrait
/// 
/// Transfer expression from a video to a portrait.
/// 
/// Category: image-to-image
/// Machine Type: A6000
/// License Type: commercial
                pub fn image(params: LivePortraitImageInput) -> FalRequest<LivePortraitImageInput, LivePortraitImageOutput> {
                    FalRequest::new(
                        "fal-ai/live-portrait/image",
                        params
                    )
                }
                