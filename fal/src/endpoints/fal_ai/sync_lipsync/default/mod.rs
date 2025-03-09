use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LipSyncOutput {
    /// The generated video/// The generated video/// {"url":"https://v3.fal.media/files/rabbit/6gJV-z7RJsF0AxkZHkdgJ_output.mp4"}
    pub video: File,
}

/// sync.so -- lipsync 1.9.0-beta
///
/// Generate realistic lipsync animations from audio using advanced algorithms for high-quality synchronization.
///
/// Category: video-to-video
///
/// License Type: commercial
pub fn default(params: LipSyncInput) -> FalRequest<LipSyncInput, LipSyncOutput> {
    FalRequest::new("fal-ai/sync-lipsync", params)
}
