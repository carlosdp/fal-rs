#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_veo2",
    feature = "endpoints_fal-ai_veo2_image-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_veo2",
        feature = "endpoints_fal-ai_veo2_image-to-video"
    )))
)]
pub mod image_to_video;

#[derive(Debug, Serialize, Deserialize)]
pub struct TextToVideoOutput {
    /// The generated video/// The generated video/// {"url":"https://v3.fal.media/files/tiger/83-YzufmOlsnhqq5ed382_output.mp4"}
    pub video: File,
}

/// Veo 2
///
/// Category: text-to-video
///
///
///
/// Generate videos using Google's [Veo 2 text-to-video model](https://blog.google/technology/google-labs/video-image-generation-update-december-2024/).
///
/// For best results, prompts should be descriptive and clear. Include:
/// - Subject: What you want in the video (object, person, animal, scenery)
/// - Context: The background/setting
/// - Action: What the subject is doing
/// - Style: Film style keywords (horror, noir, cartoon etc.)
/// - Camera motion (optional): aerial view, tracking shot etc.
/// - Composition (optional): wide shot, close-up etc.
/// - Ambiance (optional): Color and lighting details
///
/// More details are available in our [prompting guide](https://blog.fal.ai/mastering-video-generation-with-veo-2-a-comprehensive-guide/).
///
/// The model supports:
/// - 720p resolution videos
/// - 5-8 second duration at 24 FPS
/// - Both 16:9 (landscape) and 9:16 (portrait) aspect ratios
///
/// Safety filters prevent generation of inappropriate content.
pub fn veo2(params: TextToVideoInput) -> FalRequest<TextToVideoInput, TextToVideoOutput> {
    FalRequest::new("fal-ai/veo2", params)
}
