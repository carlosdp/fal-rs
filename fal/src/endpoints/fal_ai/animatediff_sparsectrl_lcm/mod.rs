#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatediffLCMOutput {
    /// The seed used to generate the video.
    pub seed: i64,
    /// Generated video file.
    pub video: File,
}

/// Animatediff SparseCtrl LCM
///
/// Category: text-to-video
/// Machine Type: A100
pub fn animatediff_sparsectrl_lcm(
    params: AnimatediffLCMInput,
) -> FalRequest<AnimatediffLCMInput, AnimatediffLCMOutput> {
    FalRequest::new("fal-ai/animatediff-sparsectrl-lcm", params)
}
