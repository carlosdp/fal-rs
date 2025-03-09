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
    

                /// DeepSeek Janus-Pro
/// 
/// DeepSeek Janus-Pro is a novel text-to-image model that unifies multimodal understanding and generation through an autoregressive framework
/// 
/// Category: text-to-image
                pub fn janus(params: JanusInput) -> FalRequest<JanusInput, Output> {
                    FalRequest::new(
                        "fal-ai/janus",
                        params
                    )
                }
                