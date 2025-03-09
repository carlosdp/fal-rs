#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Recraft20BTextToImageOutput {
    pub images: Vec<File>,
}

/// Recraft 20b
///
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
///
/// State of the art Recraft 20B model by recraft.ai, use it as an API directly through fal.
pub fn recraft_20b(
    params: Recraft20BTextToImageInput,
) -> FalRequest<Recraft20BTextToImageInput, Recraft20BTextToImageOutput> {
    FalRequest::new("fal-ai/recraft-20b", params)
}
