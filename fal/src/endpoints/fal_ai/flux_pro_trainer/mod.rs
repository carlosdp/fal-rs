#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct FluxFinetuneOutput {
    /// References your specific model
    pub finetune_id: String,
}

/// Train Flux LoRAs For Pro Models
///
/// Category: training
/// Machine Type: H100
///
///
/// FLUX.1 Finetune [pro] API, next generation text-to-image model.
///
/// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
pub fn flux_pro_trainer(
    params: FluxFinetuneInput,
) -> FalRequest<FluxFinetuneInput, FluxFinetuneOutput> {
    FalRequest::new("fal-ai/flux-pro-trainer", params)
}
