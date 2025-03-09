#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// URL to the caption .mp4 video.
    pub video_url: String,
}

/// Auto-Captioner
///
/// Category: video-to-video
pub fn auto_caption(params: CaptionInput) -> FalRequest<CaptionInput, Output> {
    FalRequest::new("fal-ai/auto-caption", params)
}
