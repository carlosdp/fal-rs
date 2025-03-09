#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct GenFillOutput {
    /// Generated Images/// Generated Images/// [{"content_type":"image/png","file_name":"a0d138e6820c4ad58f1fd3c758f16047.png","file_size":1064550,"height":768,"url":"https://storage.googleapis.com/falserverless/bria/bria_genfill_res.png","width":1024}]
    pub images: Vec<Image>,
}

/// Bria Text-to-Image Base
///
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
pub fn genfill(params: GenFillInput) -> FalRequest<GenFillInput, GenFillOutput> {
    FalRequest::new("fal-ai/bria/genfill", params)
}
