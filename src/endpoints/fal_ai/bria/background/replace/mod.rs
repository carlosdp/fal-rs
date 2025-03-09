#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct BGReplaceOutput {
    /// The generated images
    /// [{"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_bg_replace_res.jpg"}]
    pub images: Vec<Image>,
    /// Seed value used for generation.
    pub seed: i64,
}

/// Bria Text-to-Image Base
///
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
pub fn replace(params: BGReplaceInput) -> FalRequest<BGReplaceInput, BGReplaceOutput> {
    FalRequest::new("fal-ai/bria/background/replace", params)
}
