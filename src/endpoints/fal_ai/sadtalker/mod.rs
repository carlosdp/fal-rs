#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_sadtalker",
    feature = "endpoints_fal-ai_sadtalker_reference"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_sadtalker",
        feature = "endpoints_fal-ai_sadtalker_reference"
    )))
)]
pub mod reference;

#[derive(Debug, Serialize, Deserialize)]
pub struct SadTalkerOutput {
    /// URL of the generated video
    pub video: File,
}

/// Sad Talker
///
/// Category: image-to-video
/// Machine Type: A100
pub fn sadtalker(params: SadTalkerInput) -> FalRequest<SadTalkerInput, SadTalkerOutput> {
    FalRequest::new("fal-ai/sadtalker", params)
}
