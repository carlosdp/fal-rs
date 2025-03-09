#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// Upscaled image
    pub image: Image,
}

/// DRCT-Super-Resolution
///
/// Category: image-to-image
/// Machine Type: A100
pub fn drct_super_resolution(params: Input) -> FalRequest<Input, Output> {
    FalRequest::new("fal-ai/drct-super-resolution", params)
}
