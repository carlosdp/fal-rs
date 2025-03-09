#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_luma-dream-machine",
    feature = "endpoints_fal-ai_luma-dream-machine_image-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_luma-dream-machine",
        feature = "endpoints_fal-ai_luma-dream-machine_image-to-video"
    )))
)]
pub mod image_to_video;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_luma-dream-machine",
    feature = "endpoints_fal-ai_luma-dream-machine_ray-2"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_luma-dream-machine",
        feature = "endpoints_fal-ai_luma-dream-machine_ray-2"
    )))
)]
pub mod ray_2;

#[derive(Debug, Serialize, Deserialize)]
pub struct T2VOutput {
    /// The generated video/// The generated video/// {"url":"https://v2.fal.media/files/807e842c734f4127a36de9262a2d292c_output.mp4"}
    pub video: File,
}

/// Luma Ray 2
///
/// Category: text-to-video
/// Machine Type: A100
pub fn luma_dream_machine(params: TextToVideoRequest) -> FalRequest<TextToVideoRequest, T2VOutput> {
    FalRequest::new("fal-ai/luma-dream-machine", params)
}
