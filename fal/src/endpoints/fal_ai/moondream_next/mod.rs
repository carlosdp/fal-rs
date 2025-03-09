#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod batch;
pub mod detection;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MoonDreamOutput {
        /// Response from the model
pub output: String
    }
    

                /// MoonDreamNext
/// 
/// MoonDreamNext is a multimodal vision-language model for captioning, gaze detection, bbox detection, point detection, and more.
/// 
/// Category: vision
/// 
/// License Type: commercial
                pub fn moondream_next(params: QueryInput) -> FalRequest<QueryInput, MoonDreamOutput> {
                    FalRequest::new(
                        "fal-ai/moondream-next",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MoonDreamOutput {
        /// Response from the model
pub output: String
    }
    

                /// MoonDreamNext Detection
/// 
/// MoonDreamNext Detection is a multimodal vision-language model for gaze detection, bbox detection, point detection, and more.
/// 
/// Category: image-to-image
/// 
/// License Type: commercial
                pub fn moondream_next(params: QueryInput) -> FalRequest<QueryInput, MoonDreamOutput> {
                    FalRequest::new(
                        "fal-ai/moondream-next/detection",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MoonDreamOutput {
        /// Response from the model
pub output: String
    }
    

                /// MoonDreamNext Batch
/// 
/// MoonDreamNext Batch is a multimodal vision-language model for batch captioning.
/// 
/// Category: vision
/// 
/// License Type: commercial
                pub fn moondream_next(params: QueryInput) -> FalRequest<QueryInput, MoonDreamOutput> {
                    FalRequest::new(
                        "fal-ai/moondream-next/batch",
                        params
                    )
                }
                