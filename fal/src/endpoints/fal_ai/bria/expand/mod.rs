#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageExpansionOutput {
    /// The generated image/// The generated image/// {"content_type":"image/png","file_name":"afa402a35ea742cdb5c3e219b2b19bfb.png","file_size":1471342,"height":674,"url":"https://v3.fal.media/files/koala/8np-spgxxG-I1r3cjthRV_afa402a35ea742cdb5c3e219b2b19bfb.png","width":1200}
    pub image: Image,
    /// Seed value used for generation.
    pub seed: i64,
}

/// Bria Text-to-Image Base
///
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
pub fn expand(
    params: ImageExpansionInput,
) -> FalRequest<ImageExpansionInput, ImageExpansionOutput> {
    FalRequest::new("fal-ai/bria/expand", params)
}
