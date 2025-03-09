#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// URL to the training configuration file.
pub config_file: File,
/// URL to the trained diffusers lora weights.
pub diffusers_lora_file: File
    }
    

                /// Train Flux LoRAs For Portraits
/// 
/// FLUX LoRA training optimized for portrait generation, with bright highlights, excellent prompt following and highly detailed results.
/// 
/// Category: training
/// Machine Type: H100
                pub fn flux_lora_portrait_trainer(params: PublicInput) -> FalRequest<PublicInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-lora-portrait-trainer",
                        params
                    )
                }
                