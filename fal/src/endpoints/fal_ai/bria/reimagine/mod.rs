#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReimagineOutput {
    /// The generated images/// The generated images/// [{"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_reimagine_output.png"}]
    pub images: Vec<Image>,
    /// Seed value used for generation.
    pub seed: i64,
}

/// Bria Text-to-Image Base
///
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
pub fn reimagine(params: ReimagineInput) -> FalRequest<ReimagineInput, ReimagineOutput> {
    FalRequest::new("fal-ai/bria/text-to-image/base", params)
}
