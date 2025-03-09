#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_live-portrait",
    feature = "endpoints_fal-ai_live-portrait_image"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_live-portrait",
        feature = "endpoints_fal-ai_live-portrait_image"
    )))
)]
pub mod image;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_live-portrait",
    feature = "endpoints_fal-ai_live-portrait_video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_live-portrait",
        feature = "endpoints_fal-ai_live-portrait_video"
    )))
)]
pub mod video;

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
pub fn live_portrait(
    params: LivePortraitInput,
) -> FalRequest<LivePortraitInput, LivePortraitOutput> {
    FalRequest::new("fal-ai/live-portrait", params)
}
