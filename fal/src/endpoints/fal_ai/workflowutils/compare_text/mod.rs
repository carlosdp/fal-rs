#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TextOutput {
        /// The output text/// The output text/// "Hello, World!"

pub text: String
    }
    

                /// Image Preprocessors
/// 
/// Various image preprocessing tools for ControlNet and other applications.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn compare_text(params: CompareTextInput) -> FalRequest<CompareTextInput, TextOutput> {
                    FalRequest::new(
                        "fal-ai/workflowutils/canny",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TextOutput {
        /// The output text/// The output text/// "Hello, World!"

pub text: String
    }
    

                /// Image Preprocessors
/// 
/// Canny edge detection preprocessor.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn compare_text(params: CompareTextInput) -> FalRequest<CompareTextInput, TextOutput> {
                    FalRequest::new(
                        "fal-ai/workflowutils/canny",
                        params
                    )
                }
                