#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NafnetOutputDenoise {
        /// The generated image file info./// The generated image file info./// {"content_type":"image/png","file_name":"36d3ca4791a647678b2ff01a35c87f5a.png","file_size":423052,"height":512,"url":"https://storage.googleapis.com/falserverless/nafnet/7c97e55956324a7cbee00ac9652a931b.png","width":512}

pub image: Image
    }
    

                /// NAFNet-deblur
/// 
/// Use NAFNet to fix issues like blurriness and noise in your images. This model specializes in image restoration and can help enhance the overall quality of your photography.
/// 
/// Category: image-to-image
/// Machine Type: A100
                pub fn denoise(params: NafnetInputDenoise) -> FalRequest<NafnetInputDenoise, NafnetOutputDenoise> {
                    FalRequest::new(
                        "fal-ai/nafnet/deblur",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NafnetOutputDenoise {
        /// The generated image file info./// The generated image file info./// {"content_type":"image/png","file_name":"36d3ca4791a647678b2ff01a35c87f5a.png","file_size":423052,"height":512,"url":"https://storage.googleapis.com/falserverless/nafnet/7c97e55956324a7cbee00ac9652a931b.png","width":512}

pub image: Image
    }
    

                /// NAFNet-denoise
/// 
/// Use NAFNet to fix issues like blurriness and noise in your images. This model specializes in image restoration and can help enhance the overall quality of your photography.
/// 
/// Category: image-to-image
/// Machine Type: A100
                pub fn denoise(params: NafnetInputDenoise) -> FalRequest<NafnetInputDenoise, NafnetOutputDenoise> {
                    FalRequest::new(
                        "fal-ai/nafnet/denoise",
                        params
                    )
                }
                