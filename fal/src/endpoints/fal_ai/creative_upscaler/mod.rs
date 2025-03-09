#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreativeUpscalerOutput {
    /// The generated image file info.
    pub image: Image,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
}

/// Creative Upscaler
///
/// Category: image-to-image
/// Machine Type: A100
pub fn creative_upscaler(
    params: CreativeUpscalerInput,
) -> FalRequest<CreativeUpscalerInput, CreativeUpscalerOutput> {
    FalRequest::new("fal-ai/creative-upscaler", params)
}
