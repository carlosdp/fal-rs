#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct EraserOutput {
    /// The generated image/// The generated image/// {"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_eraser_res.png"}
    pub image: Image,
}

/// Bria Text-to-Image Base
///
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
pub fn eraser(params: EraserInput) -> FalRequest<EraserInput, EraserOutput> {
    FalRequest::new("fal-ai/bria/text-to-image/base", params)
}
