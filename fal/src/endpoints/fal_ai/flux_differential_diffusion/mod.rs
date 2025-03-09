#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image files info.
pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// FLUX.1 [dev] Differential Diffusion
/// 
/// FLUX.1 Differential Diffusion is a rapid endpoint that enables swift, granular control over image transformations through change maps, delivering fast and precise region-specific modifications while maintaining FLUX.1 [dev]'s high-quality output.
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
                pub fn flux_differential_diffusion(params: DiffInput) -> FalRequest<DiffInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-differential-diffusion",
                        params
                    )
                }
                