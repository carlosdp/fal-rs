#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct LLavaOutput {
    /// Generated output/// Generated output/// "Leonardo da Vinci"
    pub output: String,
    /// Whether the output is partial
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<bool>,
}

/// LLaVA v1.6 34B
///
/// Category: vision
/// Machine Type: A100
/// License Type: research
pub fn llava_next(params: LLavaInput) -> FalRequest<LLavaInput, LLavaOutput> {
    FalRequest::new("fal-ai/llava-next", params)
}
