#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct DocResOutput {
    /// The generated image file info./// The generated image file info./// {"content_type":"image/png","file_name":"36d3ca4791a647678b2ff01a35c87f5a.png","file_size":423052,"height":512,"url":"https://storage.googleapis.com/falserverless/docres_ckpt/Xssvg5K39QiD6mn9K5toF_f4942abeef8d4c7bbe236b59aed5e382.png","width":512}
    pub image: Image,
}

/// DocRes
///
/// Category: image-to-image
/// Machine Type: A100
pub fn dewarp(params: DocResInputDewarp) -> FalRequest<DocResInputDewarp, DocResOutput> {
    FalRequest::new("fal-ai/docres/dewarp", params)
}
