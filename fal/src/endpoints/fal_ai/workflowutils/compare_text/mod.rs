#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TextOutput {
    /// The output text/// The output text/// "Hello, World!"
    pub text: String,
}

/// Image Preprocessors
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn compare_text(params: CompareTextInput) -> FalRequest<CompareTextInput, TextOutput> {
    FalRequest::new("fal-ai/workflowutils/canny", params)
}
