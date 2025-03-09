#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct DDColorOutput {
    /// The generated image file info.
    /// {"content_type":"image/png","file_name":"36d3ca4791a647678b2ff01a35c87f5a.png","file_size":423052,"height":512,"url":"https://storage.googleapis.com/falserverless/gallery/5fcaaac6d1344d998ebb9703102c6c63.png","width":512}
    pub image: Image,
}

/// DDColor
///
/// Category: image-to-image
/// Machine Type: A100
pub fn ddcolor(params: DDColorInput) -> FalRequest<DDColorInput, DDColorOutput> {
    FalRequest::new("fal-ai/ddcolor", params)
}
