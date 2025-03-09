#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

pub mod american_english;
pub mod brazilian_portuguese;
pub mod british_english;
pub mod french;
pub mod hindi;
pub mod italian;
pub mod japanese;
pub mod mandarin_chinese;
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
    FalRequest::new("fal-ai/kokoro/american-english", params)
}
