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
/// Ideogram's state-of-the-art image editing model. Can be used as an API directly from fal.
pub fn edit(params: EditImageInput) -> FalRequest<EditImageInput, Output> {
    FalRequest::new("fal-ai/ideogram/v2", params)
}
