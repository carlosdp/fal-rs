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

/// LLaVA v1.5 13B
///
/// Category: vision
/// Machine Type: A6000
pub fn llavav15_13b(params: LLavaInput) -> FalRequest<LLavaInput, LLavaOutput> {
    FalRequest::new("fal-ai/llavav15-13b", params)
}
