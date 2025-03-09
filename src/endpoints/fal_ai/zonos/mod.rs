#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZonosOutput {
    /// The generated audio
    pub audio: File,
}

/// Zonos-Audio-Clone
///
/// Category: text-to-audio
/// Machine Type: A100
pub fn zonos(params: ZonosInput) -> FalRequest<ZonosInput, ZonosOutput> {
    FalRequest::new("fal-ai/zonos", params)
}
