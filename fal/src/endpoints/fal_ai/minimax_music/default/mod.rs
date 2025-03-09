use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MusicOutput {
    /// The generated music/// The generated music/// {"url":"https://fal.media/files/elephant/N5UNLCwkC2y8v7a3LQLFE_output.mp3"}
    pub audio: File,
}

/// MiniMax (Hailuo AI) Music
///
/// Generate music from text prompts using the MiniMax model, which leverages advanced AI techniques to create high-quality, diverse musical compositions.
///
/// Category: text-to-audio
///
/// License Type: commercial
pub fn default(params: TextToMusicRequest) -> FalRequest<TextToMusicRequest, MusicOutput> {
    FalRequest::new("fal-ai/minimax-music", params)
}
