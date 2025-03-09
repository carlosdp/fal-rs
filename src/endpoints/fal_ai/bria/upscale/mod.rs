#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpscaleOutput {
    /// Result Image
    /// {"content_type":"image/png","file_name":"db12c5f7076844d0bb84df92ab340acd.png","file_size":2494064,"height":1400,"url":"https://v3.fal.media/files/penguin/oHW1CIjw26zf3Jt-YLBTW_db12c5f7076844d0bb84df92ab340acd.png","width":1220}
    pub image: Image,
}

/// Bria Text-to-Image Base
///
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
pub fn upscale(params: UpscaleInput) -> FalRequest<UpscaleInput, UpscaleOutput> {
    FalRequest::new("fal-ai/bria/upscale", params)
}
