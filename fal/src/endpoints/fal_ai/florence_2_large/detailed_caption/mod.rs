#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TextOutput {
    /// Results from the model
    pub results: String,
}

/// Florence-2 Large
///
/// Category: vision
/// Machine Type: A100
/// License Type: commercial
pub fn detailed_caption(params: ImageInput) -> FalRequest<ImageInput, TextOutput> {
    FalRequest::new("fal-ai/florence-2-large/caption", params)
}
