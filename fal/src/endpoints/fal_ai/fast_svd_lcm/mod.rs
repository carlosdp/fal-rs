#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod text_to_video;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FastSVDOutput {
        /// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
/// The generated video file.
pub video: File
    }
    

                /// Stable Video Diffusion Turbo
/// 
/// Generate short video clips from your images using SVD v1.1 at Lightning Speed
/// 
/// Category: image-to-video
/// Machine Type: A100
                pub fn fast_svd_lcm(params: FastSVDImageInput) -> FalRequest<FastSVDImageInput, FastSVDOutput> {
                    FalRequest::new(
                        "fal-ai/fast-svd-lcm",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FastSVDOutput {
        /// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
/// The generated video file.
pub video: File
    }
    

                /// Stable Video Diffusion Turbo
/// 
/// Generate short video clips from your images using SVD v1.1 at Lightning Speed
/// 
/// Category: text-to-video
/// Machine Type: A100
                pub fn fast_svd_lcm(params: FastSVDImageInput) -> FalRequest<FastSVDImageInput, FastSVDOutput> {
                    FalRequest::new(
                        "fal-ai/fast-svd-lcm/text-to-video",
                        params
                    )
                }
                