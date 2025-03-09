#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputModel {
    /// Generated video prompt
    /// "A futuristic city glows softly at dusk, captured with smooth gimbal movements and a slow burn pacing, enhanced by subtle holographic overlays."
    pub prompt: String,
}

/// Video Prompt Generator
///
/// Category: llm
pub fn video_prompt_generator(params: InputModel) -> FalRequest<InputModel, OutputModel> {
    FalRequest::new("fal-ai/video-prompt-generator", params)
}
