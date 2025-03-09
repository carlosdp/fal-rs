#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// URLs of the generated images
    pub images: Vec<Image>,
}

/// FASHN Virtual Try-On
///
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
pub fn tryon(params: Input) -> FalRequest<Input, Output> {
    FalRequest::new("fashn/tryon", params)
}
