#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct LivePortraitImageOutput {
    /// The generated image file.
    pub image: Image,
}

/// Live Portrait
///
/// Category: image-to-video
/// Machine Type: A6000
/// License Type: commercial
pub fn image(
    params: LivePortraitImageInput,
) -> FalRequest<LivePortraitImageInput, LivePortraitImageOutput> {
    FalRequest::new("fal-ai/live-portrait/image", params)
}
