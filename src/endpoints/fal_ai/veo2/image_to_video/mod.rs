#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageToVideoOutput {
    /// The generated video
    /// {"url":"https://v3.fal.media/files/zebra/uNu-1qkbNt8be8iHA1hiB_output.mp4"}
    pub video: File,
}

/// Veo 2
///
/// Category: text-to-video
///
///
///
/// Generate videos by animating an input image using Google's [Veo 2 model](https://blog.google/technology/google-labs/video-image-generation-update-december-2024/).
///
/// The prompt should describe how to animate the input image. Include:
/// - Action: How the image should be animated
/// - Style: Desired animation style
/// - Camera motion (optional): How camera should move
/// - Ambiance (optional): Desired mood and atmosphere
///
/// More details are available in our [prompting guide](https://blog.fal.ai/mastering-video-generation-with-veo-2-a-comprehensive-guide/).
///
/// The model supports:
/// - Input images up to 8MB in size
/// - 720p output resolution
/// - Both 16:9 (landscape) and 9:16 (portrait) aspect ratios
/// - Natural motion and realistic animations
/// - Control over animation via text prompts
///
/// Safety filters are applied to both input images and generated content.
pub fn image_to_video(
    params: ImageToVideoInput,
) -> FalRequest<ImageToVideoInput, ImageToVideoOutput> {
    FalRequest::new("fal-ai/veo2/image-to-video", params)
}
