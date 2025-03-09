#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

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
/// Faster version of Ideogram 2a Remix. Can be used as an API directly from fal.
pub fn remix(params: RemixImageInput) -> FalRequest<RemixImageInput, Output> {
    FalRequest::new("fal-ai/ideogram/v2", params)
}
