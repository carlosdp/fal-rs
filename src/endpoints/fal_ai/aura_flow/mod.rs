#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// The generated images
    pub images: Vec<Image>,
    /// The expanded prompt
    pub prompt: String,
    /// The seed used to generate the images
    pub seed: i64,
}

/// AuraFlow
///
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
pub fn aura_flow(params: Input) -> FalRequest<Input, Output> {
    FalRequest::new("fal-ai/aura-flow", params)
}
