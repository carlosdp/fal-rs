#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct FrenchOutput {
    /// The generated music
    /// {"url":"https://fal.media/files/kangaroo/E_itKJKZKRNaO-QtU77k1_tmpe1qso5xp.wav"}
    pub audio: File,
}

/// Kokoro TTS
///
/// Category: text-to-audio
/// Machine Type: A100
pub fn french(params: FrenchRequest) -> FalRequest<FrenchRequest, FrenchOutput> {
    FalRequest::new("fal-ai/kokoro/french", params)
}
