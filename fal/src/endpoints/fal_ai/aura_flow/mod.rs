#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// The generated images
pub images: Vec<Image>,
/// The expanded prompt
pub prompt: String,
/// The seed used to generate the images
pub seed: i64
    }
    

                /// AuraFlow
/// 
/// AuraFlow v0.3 is an open-source flow-based text-to-image generation model that achieves state-of-the-art results on GenEval. The model is currently in beta.
/// 
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
                pub fn aura_flow(params: Input) -> FalRequest<Input, Output> {
                    FalRequest::new(
                        "fal-ai/aura-flow",
                        params
                    )
                }
                