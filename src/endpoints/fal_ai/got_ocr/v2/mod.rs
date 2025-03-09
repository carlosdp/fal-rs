#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageChatOutput {
    /// Generated output
    pub outputs: Vec<String>,
}

/// GOT OCR 2.0
///
/// Category: vision
/// Machine Type: A100
pub fn v2(params: ImageInput) -> FalRequest<ImageInput, ImageChatOutput> {
    FalRequest::new("fal-ai/got-ocr/v2", params)
}
