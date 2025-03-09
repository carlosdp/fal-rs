#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct LivePortraitOutput {
    /// The generated video file.
    pub video: File,
}

/// Live Portrait
///
/// Category: image-to-video
/// Machine Type: A6000
/// License Type: commercial
pub fn video(
    params: LivePortraitVideoInput,
) -> FalRequest<LivePortraitVideoInput, LivePortraitOutput> {
    FalRequest::new("fal-ai/live-portrait/video", params)
}
