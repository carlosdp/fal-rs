#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BatchMoonDreamOutput {
        /// URL to the generated captions JSON file containing filename-caption pairs.
pub captions_file: File,
/// List of generated captions
pub outputs: Vec<String>
    }
    

                /// MoonDreamNext
/// 
/// MoonDreamNext is a multimodal vision-language model for captioning, gaze detection, bbox detection, point detection, and more.
/// 
/// Category: vision
/// 
/// License Type: commercial
                pub fn batch(params: BatchQueryInput) -> FalRequest<BatchQueryInput, BatchMoonDreamOutput> {
                    FalRequest::new(
                        "fal-ai/moondream-next",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BatchMoonDreamOutput {
        /// URL to the generated captions JSON file containing filename-caption pairs.
pub captions_file: File,
/// List of generated captions
pub outputs: Vec<String>
    }
    

                /// MoonDreamNext Detection
/// 
/// MoonDreamNext Detection is a multimodal vision-language model for gaze detection, bbox detection, point detection, and more.
/// 
/// Category: image-to-image
/// 
/// License Type: commercial
                pub fn batch(params: BatchQueryInput) -> FalRequest<BatchQueryInput, BatchMoonDreamOutput> {
                    FalRequest::new(
                        "fal-ai/moondream-next/detection",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BatchMoonDreamOutput {
        /// URL to the generated captions JSON file containing filename-caption pairs.
pub captions_file: File,
/// List of generated captions
pub outputs: Vec<String>
    }
    

                /// MoonDreamNext Batch
/// 
/// MoonDreamNext Batch is a multimodal vision-language model for batch captioning.
/// 
/// Category: vision
/// 
/// License Type: commercial
                pub fn batch(params: BatchQueryInput) -> FalRequest<BatchQueryInput, BatchMoonDreamOutput> {
                    FalRequest::new(
                        "fal-ai/moondream-next/batch",
                        params
                    )
                }
                