#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConformerOutput {
        /// The generated image file info./// The generated image file info./// {"content_type":"image/png","file_name":"36d3ca4791a647678b2ff01a35c87f5a.png","file_size":423052,"height":512,"url":"https://storage.googleapis.com/falserverless/model_tests/codeformer/codeformer_restored_1.jpeg","width":512}

pub image: Image,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64
    }
    

                /// CodeFormer
/// 
/// Fix distorted or blurred photos of people with CodeFormer.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn codeformer(params: CodeformerInput) -> FalRequest<CodeformerInput, ConformerOutput> {
                    FalRequest::new(
                        "fal-ai/codeformer",
                        params
                    )
                }
                