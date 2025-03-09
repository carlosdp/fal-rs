#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TryOnOutput {
        /// The output image./// The output image./// {"content_type":"image/png","file_name":"result.png","file_size":595094,"height":1024,"url":"https://v3.fal.media/files/panda/Hoy3zhimzVKi3F2uoGBnh_result.png","width":768}

pub image: Image
    }
    

                /// Kling Kolors Virtual TryOn v1.5
/// 
/// Kling Kolors Virtual TryOn v1.5 is a high quality image based Try-On endpoint which can be used for commercial try on.
/// 
/// Category: image-to-image
/// Machine Type: A100
/// License Type: commercial
/// 
/// Kling Kolors Virtual Try-On
                pub fn kolors_virtual_try_on(params: TryOnRequest) -> FalRequest<TryOnRequest, TryOnOutput> {
                    FalRequest::new(
                        "fal-ai/kling/v1-5/kolors-virtual-try-on",
                        params
                    )
                }
                