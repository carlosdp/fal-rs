#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectOutput {
    /// Generated 3D mesh file
    pub model_mesh: File,
    /// Processing timings
    pub timings: Timings,
}

/// Trellis
///
/// Category: image-to-3d
/// Machine Type: A100
/// License Type: commercial
pub fn trellis(params: InputModel) -> FalRequest<InputModel, ObjectOutput> {
    FalRequest::new("fal-ai/trellis", params)
}
