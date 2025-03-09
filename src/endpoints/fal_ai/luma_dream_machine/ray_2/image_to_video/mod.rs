#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ray2I2VOutput {
    /// URL of the generated video
    /// {"url":"https://v3.fal.media/files/zebra/9aDde3Te2kuJYHdR0Kz8R_output.mp4"}
    pub video: File,
}

/// Luma Ray 2
///
/// Category: text-to-video
/// Machine Type: A100
///
///
/// Luma's state of the art Ray2 model for image-to-video generation.
///
/// Takes initial and/or final images and generates a video that starts from
/// and/or ends with those images.
pub fn image_to_video(
    params: Ray2ImageToVideoRequest,
) -> FalRequest<Ray2ImageToVideoRequest, Ray2I2VOutput> {
    FalRequest::new("fal-ai/luma-dream-machine/ray-2/image-to-video", params)
}
