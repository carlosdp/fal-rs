#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ObjectOutput {
        /// Generated 3D object file.
pub model_mesh: File,
/// Directory containing textures for the remeshed model.
#[serde(skip_serializing_if = "Option::is_none")]
pub remeshing_dir: Option<Option<File>>,
/// Inference timings.
pub timings: Timings
    }
    

                /// TripoSR
/// 
/// State of the art Image to 3D Object generation
/// 
/// Category: image-to-3d
/// Machine Type: A6000
                pub fn remeshing(params: RemeshingInput) -> FalRequest<RemeshingInput, ObjectOutput> {
                    FalRequest::new(
                        "fal-ai/triposr",
                        params
                    )
                }
                