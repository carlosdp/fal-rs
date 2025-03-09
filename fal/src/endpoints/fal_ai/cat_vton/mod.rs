#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CATVTONOutput {
        /// The output image.
pub image: Image
    }
    

                /// try-on
/// 
/// Image based Virtual Try-On
/// 
/// Category: image-to-image
/// Machine Type: A100
/// License Type: research
                pub fn cat_vton(params: CATVTONInput) -> FalRequest<CATVTONInput, CATVTONOutput> {
                    FalRequest::new(
                        "fal-ai/cat-vton",
                        params
                    )
                }
                