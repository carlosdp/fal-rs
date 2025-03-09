#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct HindiOutput {
    /// The generated music/// The generated music/// {"url":"https://fal.media/files/elephant/3sGUskl1AFG4TN_NAinO8_tmpdq_1m8og.wav"}
    pub audio: File,
}

/// Kokoro TTS
///
/// Category: text-to-audio
/// Machine Type: A100
pub fn hindi(params: HindiRequest) -> FalRequest<HindiRequest, HindiOutput> {
    FalRequest::new("fal-ai/kokoro/hindi", params)
}
