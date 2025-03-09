#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_triposr",
    feature = "endpoints_fal-ai_triposr_remeshing"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_triposr",
        feature = "endpoints_fal-ai_triposr_remeshing"
    )))
)]
pub mod remeshing;

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
pub fn triposr(params: TripoSRInput) -> FalRequest<TripoSRInput, ObjectOutput> {
    FalRequest::new("fal-ai/triposr", params)
}
