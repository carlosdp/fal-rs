#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SadTalkerOutput {
    /// URL of the generated video
    pub video: File,
}

/// Sad Talker
///
/// Category: image-to-video
/// Machine Type: A100
pub fn reference(
    params: SadTalkerRefVideoInput,
) -> FalRequest<SadTalkerRefVideoInput, SadTalkerOutput> {
    FalRequest::new("fal-ai/sadtalker", params)
}
