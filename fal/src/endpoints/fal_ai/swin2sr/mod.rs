#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SwinSrOutput {
    /// The generated image file info.
    pub image: Image,
}

/// SWIN2SR
///
/// Category: image-to-image
/// Machine Type: A100
pub fn swin2sr(params: SwinSrInput) -> FalRequest<SwinSrInput, SwinSrOutput> {
    FalRequest::new("fal-ai/swin2sr", params)
}
