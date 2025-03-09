#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ImageChatOutput {
        /// Generated output
pub outputs: Vec<String>
    }
    

                /// GOT OCR 2.0
/// 
/// GOT-OCR2 works on a wide range of tasks, including plain document OCR, scene text OCR, formatted document OCR, and even OCR for tables, charts, mathematical formulas, geometric shapes, molecular formulas and sheet music.
/// 
/// Category: vision
/// Machine Type: A100
                pub fn v2(params: ImageInput) -> FalRequest<ImageInput, ImageChatOutput> {
                    FalRequest::new(
                        "fal-ai/got-ocr/v2",
                        params
                    )
                }
                