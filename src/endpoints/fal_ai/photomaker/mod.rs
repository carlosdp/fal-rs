#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoMakerOutput {
    pub images: Vec<Image>,
    pub seed: i64,
}

/// PhotoMaker
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn photomaker(params: PhotoMakerInput) -> FalRequest<PhotoMakerInput, PhotoMakerOutput> {
    FalRequest::new("fal-ai/photomaker", params)
}
