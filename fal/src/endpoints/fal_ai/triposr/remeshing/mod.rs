#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectOutput {
    /// Generated 3D object file.
    pub model_mesh: File,
    /// Directory containing textures for the remeshed model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remeshing_dir: Option<Option<File>>,
    /// Inference timings.
    pub timings: Timings,
}

/// TripoSR
///
/// Category: image-to-3d
/// Machine Type: A6000
pub fn remeshing(params: RemeshingInput) -> FalRequest<RemeshingInput, ObjectOutput> {
    FalRequest::new("fal-ai/triposr", params)
}
