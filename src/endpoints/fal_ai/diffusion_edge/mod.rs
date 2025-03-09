#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffusionEdgeOutput {
    /// The generated image file info.
    pub image: Image,
}

/// DiffusionEdge
///
/// Category: text-to-image
/// Machine Type: A6000
pub fn diffusion_edge(
    params: DiffusionEdgeInput,
) -> FalRequest<DiffusionEdgeInput, DiffusionEdgeOutput> {
    FalRequest::new("fal-ai/diffusion-edge", params)
}
