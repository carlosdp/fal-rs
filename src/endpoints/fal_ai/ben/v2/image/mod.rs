#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ben2OutputImage {
    /// The output image after background removal.
    /// {"content_type":"image/png","file_name":"zrZNETpI_ul2jonraqpxN_a57c3f3825d9418f8b3d39cde87c3310.png","file_size":423052,"height":512,"url":"https://storage.googleapis.com/falserverless/gallery/Ben2/zrZNETpI_ul2jonraqpxN_a57c3f3825d9418f8b3d39cde87c3310.png","width":512}
    pub image: Image,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
}

/// ben-v2-image
///
/// Category: image-to-image
/// Machine Type: A100
pub fn image(params: Ben2InputImage) -> FalRequest<Ben2InputImage, Ben2OutputImage> {
    FalRequest::new("fal-ai/ben/v2/image", params)
}
