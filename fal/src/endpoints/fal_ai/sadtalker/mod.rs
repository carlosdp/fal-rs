#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod reference;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SadTalkerOutput {
        /// URL of the generated video
pub video: File
    }
    

                /// Sad Talker
/// 
/// Learning Realistic 3D Motion Coefficients for Stylized Audio-Driven Single Image Talking Face Animation
/// 
/// Category: image-to-video
/// Machine Type: A100
                pub fn sadtalker(params: SadTalkerInput) -> FalRequest<SadTalkerInput, SadTalkerOutput> {
                    FalRequest::new(
                        "fal-ai/sadtalker",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SadTalkerOutput {
        /// URL of the generated video
pub video: File
    }
    

                /// Sad Talker
/// 
/// Learning Realistic 3D Motion Coefficients for Stylized Audio-Driven Single Image Talking Face Animation
/// 
/// Category: image-to-video
/// Machine Type: A100
                pub fn sadtalker(params: SadTalkerInput) -> FalRequest<SadTalkerInput, SadTalkerOutput> {
                    FalRequest::new(
                        "fal-ai/sadtalker/reference",
                        params
                    )
                }
                