#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod frame_interpolation;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AMTInterpolationOutput {
        /// Generated video
pub video: File
    }
    

                /// AMT Interpolation
/// 
/// Interpolate between video frames
/// 
/// Category: video-to-video
/// Machine Type: A6000
                pub fn amt_interpolation(params: AMTInterpolationInput) -> FalRequest<AMTInterpolationInput, AMTInterpolationOutput> {
                    FalRequest::new(
                        "fal-ai/amt-interpolation",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AMTInterpolationOutput {
        /// Generated video
pub video: File
    }
    

                /// AMT Frame Interpolation
/// 
/// Interpolate between image frames
/// 
/// Category: image-to-video
/// Machine Type: A6000
                pub fn amt_interpolation(params: AMTInterpolationInput) -> FalRequest<AMTInterpolationInput, AMTInterpolationOutput> {
                    FalRequest::new(
                        "fal-ai/amt-interpolation/frame-interpolation",
                        params
                    )
                }
                