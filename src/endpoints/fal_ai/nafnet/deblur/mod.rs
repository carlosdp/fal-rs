#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct NafnetOutput {
    /// The generated image file info./// The generated image file info./// {"content_type":"image/png","file_name":"36d3ca4791a647678b2ff01a35c87f5a.png","file_size":423052,"height":512,"url":"https://storage.googleapis.com/falserverless/nafnet/2cbfd460e25344a69fa8077808fb484f.png","width":512}
    pub image: Image,
}

/// NAFNet-deblur
///
/// Category: image-to-image
/// Machine Type: A100
pub fn deblur(params: NafnetInput) -> FalRequest<NafnetInput, NafnetOutput> {
    FalRequest::new("fal-ai/nafnet/deblur", params)
}
