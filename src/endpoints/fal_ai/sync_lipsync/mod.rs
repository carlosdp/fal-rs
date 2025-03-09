#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct LipSyncOutput {
    /// The generated video/// The generated video/// {"url":"https://v3.fal.media/files/rabbit/6gJV-z7RJsF0AxkZHkdgJ_output.mp4"}
    pub video: File,
}

/// sync.so -- lipsync 1.9.0-beta
///
/// Category: video-to-video
///
/// License Type: commercial
pub fn sync_lipsync(params: LipSyncInput) -> FalRequest<LipSyncInput, LipSyncOutput> {
    FalRequest::new("fal-ai/sync-lipsync", params)
}
