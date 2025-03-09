#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod create_style;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TextToImageOutput {
        pub images: Vec<File>
    }
    

                /// Recraft V3
/// 
/// Recraft V3 is a text-to-image model with the ability to generate long texts, vector art, images in brand style, and much more. As of today, it is SOTA in image generation, proven by Hugging Face's industry-leading Text-to-Image Benchmark by Artificial Analysis.
/// 
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
/// 
/// State of the art Recraft V3 model by recraft.ai, use it as an API directly through fal.
                pub fn recraft_v3(params: TextToImageInput) -> FalRequest<TextToImageInput, TextToImageOutput> {
                    FalRequest::new(
                        "fal-ai/recraft-v3",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TextToImageOutput {
        pub images: Vec<File>
    }
    

                /// Recraft V3 - Create Style
/// 
/// Recraft V3 Create Style is capable of creating unique styles for Recraft V3 based on your images.
/// 
/// Category: training
/// Machine Type: A100
/// License Type: commercial
/// 
/// State of the art Recraft V3 model by recraft.ai, use it as an API directly through fal.
                pub fn recraft_v3(params: TextToImageInput) -> FalRequest<TextToImageInput, TextToImageOutput> {
                    FalRequest::new(
                        "fal-ai/recraft-v3/create-style",
                        params
                    )
                }
                