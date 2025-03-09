#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MusicOutput {
    /// The generated music
    /// {"url":"https://fal.media/files/elephant/N5UNLCwkC2y8v7a3LQLFE_output.mp3"}
    pub audio: File,
}

/// MiniMax (Hailuo AI) Music
///
/// Category: text-to-audio
///
/// License Type: commercial
pub fn minimax_music(params: TextToMusicRequest) -> FalRequest<TextToMusicRequest, MusicOutput> {
    FalRequest::new("fal-ai/minimax-music", params)
}
