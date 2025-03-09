#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RetoucherOutput {
        /// The generated image file info./// The generated image file info./// {"url":"https://storage.googleapis.com/falserverless/model_tests/retoucher/retoucher_example_output.png"}

pub image: Image,
/// The seed used for the generation.
pub seed: i64
    }
    

                /// Face Retoucher
/// 
/// Automatically retouches faces to smooth skin and remove blemishes.
/// 
/// Category: image-to-image
/// Machine Type: A100
                pub fn retoucher(params: RetoucherInput) -> FalRequest<RetoucherInput, RetoucherOutput> {
                    FalRequest::new(
                        "fal-ai/retoucher",
                        params
                    )
                }
                