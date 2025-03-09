#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TeedOutput {
        /// The edge map./// The edge map./// {"content_type":"image/png","height":2048,"url":"https://storage.googleapis.com/falserverless/model_tests/workflow_utils/teed_output.png","width":1246}

pub image: Image
    }
    

                /// Image Preprocessors
/// 
/// Various image preprocessing tools for ControlNet and other applications.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn teed(params: TeedInput) -> FalRequest<TeedInput, TeedOutput> {
                    FalRequest::new(
                        "fal-ai/workflowutils/canny",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TeedOutput {
        /// The edge map./// The edge map./// {"content_type":"image/png","height":2048,"url":"https://storage.googleapis.com/falserverless/model_tests/workflow_utils/teed_output.png","width":1246}

pub image: Image
    }
    

                /// Image Preprocessors
/// 
/// Canny edge detection preprocessor.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn teed(params: TeedInput) -> FalRequest<TeedInput, TeedOutput> {
                    FalRequest::new(
                        "fal-ai/workflowutils/canny",
                        params
                    )
                }
                