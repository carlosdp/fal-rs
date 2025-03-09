#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// The URL to the generated video
    pub video: File,
}

/// T2V Turbo - Video Crafter
///
/// Category: text-to-video
/// Machine Type: A100
pub fn t2v_turbo(params: Input) -> FalRequest<Input, Output> {
    FalRequest::new("fal-ai/t2v-turbo", params)
}
