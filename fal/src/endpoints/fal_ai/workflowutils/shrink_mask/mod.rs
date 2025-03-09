#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ShrinkMaskOutput {
        /// The mask/// The mask/// {"content_type":"image/png","height":700,"url":"https://storage.googleapis.com/falserverless/model_tests/workflow_utils/shrink_mask_output.png","width":610}

pub image: Image
    }
    

                /// Image Preprocessors
/// 
/// Various image preprocessing tools for ControlNet and other applications.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn shrink_mask(params: ShrinkMaskInput) -> FalRequest<ShrinkMaskInput, ShrinkMaskOutput> {
                    FalRequest::new(
                        "fal-ai/workflowutils/canny",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ShrinkMaskOutput {
        /// The mask/// The mask/// {"content_type":"image/png","height":700,"url":"https://storage.googleapis.com/falserverless/model_tests/workflow_utils/shrink_mask_output.png","width":610}

pub image: Image
    }
    

                /// Image Preprocessors
/// 
/// Canny edge detection preprocessor.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn shrink_mask(params: ShrinkMaskInput) -> FalRequest<ShrinkMaskInput, ShrinkMaskOutput> {
                    FalRequest::new(
                        "fal-ai/workflowutils/canny",
                        params
                    )
                }
                