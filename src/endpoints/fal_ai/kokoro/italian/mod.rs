#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ItalianOutput {
    /// The generated music/// The generated music/// {"url":"https://fal.media/files/monkey/-MZ0hRO4IpTMukb_S5aRZ_tmpin14eoed.wav"}
    pub audio: File,
}

/// Kokoro TTS
///
/// Category: text-to-audio
/// Machine Type: A100
pub fn italian(params: ItalianRequest) -> FalRequest<ItalianRequest, ItalianOutput> {
    FalRequest::new("fal-ai/kokoro/italian", params)
}
