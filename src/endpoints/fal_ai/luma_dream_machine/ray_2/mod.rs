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
    feature = "endpoints_fal-ai_luma-dream-machine_ray-2",
    feature = "endpoints_fal-ai_luma-dream-machine_ray-2_image-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_luma-dream-machine",
        feature = "endpoints_fal-ai_luma-dream-machine_ray-2",
        feature = "endpoints_fal-ai_luma-dream-machine_ray-2_image-to-video"
    )))
)]
pub mod image_to_video;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ray2T2VOutput {
    /// The generated video
    /// {"url":"https://v3.fal.media/files/penguin/Om3xjcOwiSCJwrXs7DUi__output.mp4"}
    pub video: File,
}

/// Luma Ray 2
///
/// Category: text-to-video
/// Machine Type: A100
///
///
/// Luma's state of the art Ray2 model for text-to-video generation.
///
/// Large-scale video generative model capable of creating realistic visuals with natural, coherent
/// motion. It has strong understanding of text instructions and can take image and video as input.
pub fn ray_2(params: Ray2TextToVideoRequest) -> FalRequest<Ray2TextToVideoRequest, Ray2T2VOutput> {
    FalRequest::new("fal-ai/luma-dream-machine/ray-2", params)
}
