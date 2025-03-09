#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AnimatediffLCMOutput {
        /// The seed used to generate the video.
pub seed: i64,
/// Generated video file.
pub video: File
    }
    

                /// Animatediff SparseCtrl LCM
/// 
/// Animate Your Drawings with Latent Consistency Models!
/// 
/// Category: text-to-video
/// Machine Type: A100
                pub fn animatediff_sparsectrl_lcm(params: AnimatediffLCMInput) -> FalRequest<AnimatediffLCMInput, AnimatediffLCMOutput> {
                    FalRequest::new(
                        "fal-ai/animatediff-sparsectrl-lcm",
                        params
                    )
                }
                