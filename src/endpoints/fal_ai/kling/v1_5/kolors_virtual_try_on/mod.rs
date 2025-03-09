#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TryOnOutput {
    /// The output image.
    /// {"content_type":"image/png","file_name":"result.png","file_size":595094,"height":1024,"url":"https://v3.fal.media/files/panda/Hoy3zhimzVKi3F2uoGBnh_result.png","width":768}
    pub image: Image,
}

/// Kling Kolors Virtual TryOn v1.5
///
/// Category: image-to-image
/// Machine Type: A100
/// License Type: commercial
///
/// Kling Kolors Virtual Try-On
pub fn kolors_virtual_try_on(params: TryOnRequest) -> FalRequest<TryOnRequest, TryOnOutput> {
    FalRequest::new("fal-ai/kling/v1-5/kolors-virtual-try-on", params)
}
