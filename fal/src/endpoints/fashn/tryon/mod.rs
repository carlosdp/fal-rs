#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// URLs of the generated images
pub images: Vec<Image>
    }
    

                /// FASHN Virtual Try-On
/// 
/// FASHN delivers precise virtual try-on capabilities, accurately rendering garment details like text and patterns at 576x864 resolution from both on-model and flat-lay photo references.
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
                pub fn tryon(params: Input) -> FalRequest<Input, Output> {
                    FalRequest::new(
                        "fashn/tryon",
                        params
                    )
                }
                