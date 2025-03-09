#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ObjectOutput {
        /// Generated 3D mesh file
pub model_mesh: File,
/// Processing timings
pub timings: Timings
    }
    

                /// Trellis
/// 
/// Generate 3D models from your images using Trellis. A native 3D generative model enabling versatile and high-quality 3D asset creation.
/// 
/// Category: image-to-3d
/// Machine Type: A100
/// License Type: commercial
                pub fn trellis(params: InputModel) -> FalRequest<InputModel, ObjectOutput> {
                    FalRequest::new(
                        "fal-ai/trellis",
                        params
                    )
                }
                