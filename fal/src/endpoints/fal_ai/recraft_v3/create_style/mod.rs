#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StyleReferenceOutput {
        /// The ID of the created style, this ID can be used to reference the style in the future.
pub style_id: String
    }
    

                /// Recraft V3
/// 
/// Recraft V3 is a text-to-image model with the ability to generate long texts, vector art, images in brand style, and much more. As of today, it is SOTA in image generation, proven by Hugging Face's industry-leading Text-to-Image Benchmark by Artificial Analysis.
/// 
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
                pub fn create_style(params: StyleReferenceInput) -> FalRequest<StyleReferenceInput, StyleReferenceOutput> {
                    FalRequest::new(
                        "fal-ai/recraft-v3",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StyleReferenceOutput {
        /// The ID of the created style, this ID can be used to reference the style in the future.
pub style_id: String
    }
    

                /// Recraft V3 - Create Style
/// 
/// Recraft V3 Create Style is capable of creating unique styles for Recraft V3 based on your images.
/// 
/// Category: training
/// Machine Type: A100
/// License Type: commercial
                pub fn create_style(params: StyleReferenceInput) -> FalRequest<StyleReferenceInput, StyleReferenceOutput> {
                    FalRequest::new(
                        "fal-ai/recraft-v3/create-style",
                        params
                    )
                }
                