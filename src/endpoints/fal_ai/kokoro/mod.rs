#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kokoro",
    feature = "endpoints_fal-ai_kokoro_american-english"
))]
pub mod american_english;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kokoro",
    feature = "endpoints_fal-ai_kokoro_brazilian-portuguese"
))]
pub mod brazilian_portuguese;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kokoro",
    feature = "endpoints_fal-ai_kokoro_british-english"
))]
pub mod british_english;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kokoro",
    feature = "endpoints_fal-ai_kokoro_french"
))]
pub mod french;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kokoro",
    feature = "endpoints_fal-ai_kokoro_hindi"
))]
pub mod hindi;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kokoro",
    feature = "endpoints_fal-ai_kokoro_italian"
))]
pub mod italian;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kokoro",
    feature = "endpoints_fal-ai_kokoro_japanese"
))]
pub mod japanese;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kokoro",
    feature = "endpoints_fal-ai_kokoro_mandarin-chinese"
))]
pub mod mandarin_chinese;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_kokoro",
    feature = "endpoints_fal-ai_kokoro_spanish"
))]
pub mod spanish;

#[derive(Debug, Serialize, Deserialize)]
pub struct AmEngOutput {
    /// The generated music/// The generated music/// {"url":"https://fal.media/files/elephant/dXVMqWsBDG9yan3kaOT0Z_tmp0vvkha3s.wav"}
    pub audio: File,
}

/// Kokoro TTS
///
/// Category: text-to-audio
/// Machine Type: A100
pub fn kokoro(params: AmEnglishRequest) -> FalRequest<AmEnglishRequest, AmEngOutput> {
    FalRequest::new("fal-ai/kokoro", params)
}
