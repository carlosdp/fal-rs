#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod video;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MiniCPMV26Output {
        /// Response from the model
pub output: String
    }
    

                /// MiniCPM-V 2.6
/// 
/// Multimodal vision-language model for single/multi image understanding
/// 
/// Category: vision
/// Machine Type: A100
/// License Type: commercial
                pub fn mini_cpm(params: MiniCPMV26ImageInput) -> FalRequest<MiniCPMV26ImageInput, MiniCPMV26Output> {
                    FalRequest::new(
                        "fal-ai/mini-cpm",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MiniCPMV26Output {
        /// Response from the model
pub output: String
    }
    

                /// MiniCPM-V 2.6
/// 
/// Multimodal vision-language model for video understanding
/// 
/// Category: vision
/// Machine Type: A100
/// License Type: research
                pub fn mini_cpm(params: MiniCPMV26ImageInput) -> FalRequest<MiniCPMV26ImageInput, MiniCPMV26Output> {
                    FalRequest::new(
                        "fal-ai/mini-cpm/video",
                        params
                    )
                }
                