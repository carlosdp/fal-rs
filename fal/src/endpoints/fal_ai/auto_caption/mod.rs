#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// URL to the caption .mp4 video.
pub video_url: String
    }
    

                /// Auto-Captioner
/// 
/// Automatically generates text captions for your videos from the audio as per text colour/font specifications
/// 
/// Category: video-to-video
                pub fn auto_caption(params: CaptionInput) -> FalRequest<CaptionInput, Output> {
                    FalRequest::new(
                        "fal-ai/auto-caption",
                        params
                    )
                }
                