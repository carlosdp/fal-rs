#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

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
