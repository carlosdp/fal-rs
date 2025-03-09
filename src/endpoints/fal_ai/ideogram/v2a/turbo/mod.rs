#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_ideogram",
    feature = "endpoints_fal-ai_ideogram_v2a",
    feature = "endpoints_fal-ai_ideogram_v2a_turbo",
    feature = "endpoints_fal-ai_ideogram_v2a_turbo_remix"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_ideogram",
        feature = "endpoints_fal-ai_ideogram_v2a",
        feature = "endpoints_fal-ai_ideogram_v2a_turbo",
        feature = "endpoints_fal-ai_ideogram_v2a_turbo_remix"
    )))
)]
pub mod remix;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub images: Vec<File>,
    /// Seed used for the random number generator/// Seed used for the random number generator/// 123456
    pub seed: i64,
}

/// Ideogram V2
///
/// Category: text-to-image
///
/// License Type: commercial
///
/// Faster version of Ideogram 2a. Can be used as an API directly from fal.
pub fn turbo(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
    FalRequest::new("fal-ai/ideogram/v2a/turbo", params)
}
