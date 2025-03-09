#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataOutput {
    /// Metadata for the analyzed media file (either Video or Audio)
    pub media: MediaProperty,
}

/// FFmpeg API Compose
///
/// Category: video-to-video
/// Machine Type: L
pub fn metadata(params: MetadataInput) -> FalRequest<MetadataInput, MetadataOutput> {
    FalRequest::new("fal-ai/ffmpeg-api/compose", params)
}
