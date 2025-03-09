#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// URL to the training configuration file.
    pub config_file: File,
    /// URL to the trained diffusers lora weights.
    pub diffusers_lora_file: File,
}

/// Train Flux LoRAs For Portraits
///
/// Category: training
/// Machine Type: H100
pub fn flux_lora_portrait_trainer(params: PublicInput) -> FalRequest<PublicInput, Output> {
    FalRequest::new("fal-ai/flux-lora-portrait-trainer", params)
}
