#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TextOutput {
    /// Results from the model
    pub results: String,
}

/// Florence-2 Large
///
/// Category: vision
/// Machine Type: A100
/// License Type: commercial
pub fn region_to_description(
    params: ImageWithUserCoordinatesInput,
) -> FalRequest<ImageWithUserCoordinatesInput, TextOutput> {
    FalRequest::new("fal-ai/florence-2-large/region-to-description", params)
}
