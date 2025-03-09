#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DiffusionEdgeOutput {
        /// The generated image file info.
pub image: Image
    }
    

                /// DiffusionEdge
/// 
/// Diffusion based high quality edge detection
/// 
/// Category: text-to-image
/// Machine Type: A6000
                pub fn diffusion_edge(params: DiffusionEdgeInput) -> FalRequest<DiffusionEdgeInput, DiffusionEdgeOutput> {
                    FalRequest::new(
                        "fal-ai/diffusion-edge",
                        params
                    )
                }
                