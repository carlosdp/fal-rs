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
    

                /// FLUX.1 Subject
/// 
/// Super fast endpoint for the FLUX.1 [schnell] model with subject input capabilities, enabling rapid and high-quality image generation for personalization, specific styles, brand identities, and product-specific outputs.
/// 
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
                pub fn flux_subject(params: FluxSubjectInput) -> FalRequest<FluxSubjectInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-subject",
                        params
                    )
                }
                