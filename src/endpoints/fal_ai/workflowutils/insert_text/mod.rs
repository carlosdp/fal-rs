#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TextOutput {
    /// The output text
    /// "Hello, World!"
    pub text: String,
}

/// Image Preprocessors
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn insert_text(params: InsertTextInput) -> FalRequest<InsertTextInput, TextOutput> {
    FalRequest::new("fal-ai/workflowutils/insert-text", params)
}
