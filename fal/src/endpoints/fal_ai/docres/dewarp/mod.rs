#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DocResOutput {
        /// The generated image file info./// The generated image file info./// {"content_type":"image/png","file_name":"36d3ca4791a647678b2ff01a35c87f5a.png","file_size":423052,"height":512,"url":"https://storage.googleapis.com/falserverless/docres_ckpt/Xssvg5K39QiD6mn9K5toF_f4942abeef8d4c7bbe236b59aed5e382.png","width":512}

pub image: Image
    }
    

                /// DocRes
/// 
/// Enhance low-resolution, blur, shadowed documents with the superior quality of docres for sharper, clearer results.
/// 
/// Category: image-to-image
/// Machine Type: A100
                pub fn dewarp(params: DocResInputDewarp) -> FalRequest<DocResInputDewarp, DocResOutput> {
                    FalRequest::new(
                        "fal-ai/docres",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DocResOutput {
        /// The generated image file info./// The generated image file info./// {"content_type":"image/png","file_name":"36d3ca4791a647678b2ff01a35c87f5a.png","file_size":423052,"height":512,"url":"https://storage.googleapis.com/falserverless/docres_ckpt/Xssvg5K39QiD6mn9K5toF_f4942abeef8d4c7bbe236b59aed5e382.png","width":512}

pub image: Image
    }
    

                /// DocRes-dewarp
/// 
/// Enhance wraped, folded documents with the superior quality of docres for sharper, clearer results.
/// 
/// Category: image-to-image
/// Machine Type: A100
                pub fn dewarp(params: DocResInputDewarp) -> FalRequest<DocResInputDewarp, DocResOutput> {
                    FalRequest::new(
                        "fal-ai/docres/dewarp",
                        params
                    )
                }
                