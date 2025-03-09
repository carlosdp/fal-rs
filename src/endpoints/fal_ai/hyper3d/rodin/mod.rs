#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectOutput {
    /// Generated 3D object file./// Generated 3D object file./// {"content_type":"application/octet-stream","file_name":"base_basic_pbr.glb","file_size":2230472,"url":"https://v3.fal.media/files/koala/VlX4JqNI8F9HO2ETp_B7t_base_basic_pbr.glb"}
    pub model_mesh: File,
    /// Seed value used for generation.
    pub seed: i64,
    /// Generated textures for the 3D object.
    pub textures: Vec<Image>,
}

/// Hyper3D Rodin
///
/// Category: image-to-3d
/// Machine Type: A100
pub fn rodin(params: Rodin3DInput) -> FalRequest<Rodin3DInput, ObjectOutput> {
    FalRequest::new("fal-ai/hyper3d/rodin", params)
}
