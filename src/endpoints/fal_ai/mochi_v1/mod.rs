#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MochiT2VOutput {
    /// The generated video/// The generated video/// {"url":"https://fal.media/files/zebra/GScPi-7ma3Fn8r1O1on4z_output_1729631871.mp4"}
    pub video: File,
}

/// Mochi 1
///
/// Category: text-to-video
/// Machine Type: A100
pub fn mochi_v1(params: MochiT2VInput) -> FalRequest<MochiT2VInput, MochiT2VOutput> {
    FalRequest::new("fal-ai/mochi-v1", params)
}
