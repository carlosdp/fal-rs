#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FlowEditOutput {
        /// The generated image file info./// The generated image file info./// {"content_type":"image/png","file_name":"36d3ca4791a647678b2ff01a35c87f5a.png","file_size":423052,"height":1024,"url":"https://storage.googleapis.com/falserverless/model_tests/FlowEdit/aa5c3d028ad04800a54f70f928198d91.png","width":1024}

pub image: Image,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64
    }
    

                /// Flow-Edit
/// 
/// The model provides you high quality image editing capabilities.
/// 
/// Category: text-to-image
/// Machine Type: A6000
                pub fn flowedit(params: FlowEditInput) -> FalRequest<FlowEditInput, FlowEditOutput> {
                    FalRequest::new(
                        "fal-ai/flowedit",
                        params
                    )
                }
                