#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DetectionOutput {
        /// Output image with detection visualization
pub image: Image,
/// Detection results as text
pub text_output: String
    }
    

                /// MoonDreamNext
/// 
/// MoonDreamNext is a multimodal vision-language model for captioning, gaze detection, bbox detection, point detection, and more.
/// 
/// Category: vision
/// 
/// License Type: commercial
                pub fn detection(params: DetectionInput) -> FalRequest<DetectionInput, DetectionOutput> {
                    FalRequest::new(
                        "fal-ai/moondream-next",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DetectionOutput {
        /// Output image with detection visualization
pub image: Image,
/// Detection results as text
pub text_output: String
    }
    

                /// MoonDreamNext Detection
/// 
/// MoonDreamNext Detection is a multimodal vision-language model for gaze detection, bbox detection, point detection, and more.
/// 
/// Category: image-to-image
/// 
/// License Type: commercial
                pub fn detection(params: DetectionInput) -> FalRequest<DetectionInput, DetectionOutput> {
                    FalRequest::new(
                        "fal-ai/moondream-next/detection",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DetectionOutput {
        /// Output image with detection visualization
pub image: Image,
/// Detection results as text
pub text_output: String
    }
    

                /// MoonDreamNext Batch
/// 
/// MoonDreamNext Batch is a multimodal vision-language model for batch captioning.
/// 
/// Category: vision
/// 
/// License Type: commercial
                pub fn detection(params: DetectionInput) -> FalRequest<DetectionInput, DetectionOutput> {
                    FalRequest::new(
                        "fal-ai/moondream-next/batch",
                        params
                    )
                }
                