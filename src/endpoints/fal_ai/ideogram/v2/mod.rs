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
    feature = "endpoints_fal-ai_ideogram_v2",
    feature = "endpoints_fal-ai_ideogram_v2_edit"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_ideogram",
        feature = "endpoints_fal-ai_ideogram_v2",
        feature = "endpoints_fal-ai_ideogram_v2_edit"
    )))
)]
pub mod edit;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_ideogram",
    feature = "endpoints_fal-ai_ideogram_v2",
    feature = "endpoints_fal-ai_ideogram_v2_remix"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_ideogram",
        feature = "endpoints_fal-ai_ideogram_v2",
        feature = "endpoints_fal-ai_ideogram_v2_remix"
    )))
)]
pub mod remix;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_ideogram",
    feature = "endpoints_fal-ai_ideogram_v2",
    feature = "endpoints_fal-ai_ideogram_v2_turbo"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_ideogram",
        feature = "endpoints_fal-ai_ideogram_v2",
        feature = "endpoints_fal-ai_ideogram_v2_turbo"
    )))
)]
pub mod turbo;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub images: Vec<File>,
    /// Seed used for the random number generator
    /// 123456
    pub seed: i64,
}

/// Ideogram V2
///
/// Category: text-to-image
///
/// License Type: commercial
///
/// Ideogram's state-of-the-art image generation model. Can be used as an API directly from fal.
pub fn v2(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
    FalRequest::new("fal-ai/ideogram/v2", params)
}
