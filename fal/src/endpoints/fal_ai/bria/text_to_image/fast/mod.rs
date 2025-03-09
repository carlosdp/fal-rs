#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// The generated images/// The generated images/// [{"content_type":"image/png","file_name":"257cf8e7bd3a47c2959396343d5b38cf.png","file_size":3731290,"height":1536,"url":"https://v3.fal.media/files/tiger/48e63e0K6C9XQYBuomoU-_257cf8e7bd3a47c2959396343d5b38cf.png","width":1536}]
    pub images: Vec<Image>,
    /// Seed value used for generation.
    pub seed: i64,
}

/// Bria Text-to-Image Base
///
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
pub fn fast(params: FastTextToImageRequest) -> FalRequest<FastTextToImageRequest, Output> {
    FalRequest::new("fal-ai/bria/text-to-image/fast", params)
}
