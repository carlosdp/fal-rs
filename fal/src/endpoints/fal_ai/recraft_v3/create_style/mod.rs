#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct StyleReferenceOutput {
    /// The ID of the created style, this ID can be used to reference the style in the future.
    pub style_id: String,
}

/// Recraft V3
///
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
pub fn create_style(
    params: StyleReferenceInput,
) -> FalRequest<StyleReferenceInput, StyleReferenceOutput> {
    FalRequest::new("fal-ai/recraft-v3", params)
}
