#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MuseTalkOutput {
    /// The generated video file.
    pub video: File,
}

/// MuseTalk
///
/// Category: image-to-video
/// Machine Type: A100
pub fn musetalk(params: MuseTalkInput) -> FalRequest<MuseTalkInput, MuseTalkOutput> {
    FalRequest::new("fal-ai/musetalk", params)
}
