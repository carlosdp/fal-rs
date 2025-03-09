#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ImageSizeOutput {
        /// Image size/// Image size/// {"height":700,"width":610}

pub image_size: HashMap<String, serde_json::Value>
    }
    

                /// Image Preprocessors
/// 
/// Various image preprocessing tools for ControlNet and other applications.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn image_size(params: ImageInput) -> FalRequest<ImageInput, ImageSizeOutput> {
                    FalRequest::new(
                        "fal-ai/workflowutils/canny",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ImageSizeOutput {
        /// Image size/// Image size/// {"height":700,"width":610}

pub image_size: HashMap<String, serde_json::Value>
    }
    

                /// Image Preprocessors
/// 
/// Canny edge detection preprocessor.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn image_size(params: ImageInput) -> FalRequest<ImageInput, ImageSizeOutput> {
                    FalRequest::new(
                        "fal-ai/workflowutils/canny",
                        params
                    )
                }
                