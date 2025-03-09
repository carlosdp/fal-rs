#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatOutput {
    /// Error message if an error occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorProperty>,
    /// Generated output/// Generated output/// "The meaning of life is subjective and depends on individual perspectives."
    pub output: String,
    /// Whether the output is partial
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<bool>,
    /// Generated reasoning for the final answer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
}

/// Any LLM
///
/// Category: llm
/// Machine Type: A6000
///
///
/// Run any vision model with fal, powered by [OpenRouter](https://openrouter.ai).
pub fn vision(params: VisionInput) -> FalRequest<VisionInput, ChatOutput> {
    FalRequest::new("fal-ai/any-llm", params)
}
