#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EyeCorrectOutput {
        /// The corrected video
pub video: File
    }
    

                /// Eye Correct
/// 
/// Eye Correct is a video-to-video model that can correct eye direction in videos. It can be used to correct eye direction in videos.
/// 
/// Category: video-to-video
/// Machine Type: A100
                pub fn eye_correct(params: EyeCorrectInput) -> FalRequest<EyeCorrectInput, EyeCorrectOutput> {
                    FalRequest::new(
                        "fal-ai/eye-correct",
                        params
                    )
                }
                