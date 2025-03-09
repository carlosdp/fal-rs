#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVoiceOutput {
    /// The S3 URI of the cloned voice.
    pub voice: String,
}

/// PlayAI Text-to-Speech v3
///
/// Category: text-to-speech
pub fn create_voice(params: CreateVoiceInput) -> FalRequest<CreateVoiceInput, CreateVoiceOutput> {
    FalRequest::new("fal-ai/playai/tts/v3", params)
}
