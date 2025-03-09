#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// The seed used for random number generation.
    pub seed: i64,
    /// The generated video.
    pub video: File,
}

/// LTX Video (preview)
///
/// Category: text-to-video
/// Machine Type: H100
/// License Type: research
///
/// LTX Video - Image to Video generation
///
/// See examples for more inspiration. Use the `image_url` parameter to provide an image to generate the video from. Make sure it is 768x512.
///
/// ### Instructions
/// When writing prompts, focus on detailed, chronological descriptions of actions and scenes. Include specific movements,
/// appearances, camera angles, and environmental details - all in a single flowing paragraph. Start directly with the action,
/// and keep descriptions literal and precise. Think like a cinematographer describing a shot list. Keep within 200 words. For
/// best results, build your prompts using this structure:
/// * Start with main action in a single sentence
/// * Add specific details about movements and gestures
/// * Describe character/object appearances precisely
/// * Include background and environment details
/// * Specify camera angles and movements
/// * Describe lighting and colors
/// * Note any changes or sudden events
///
/// ### Parameter Guide
/// * Guidance Scale: Higher values (5-7) for accurate prompt following, lower values (3-5) for more creative freedom
/// * Inference Steps: More steps (40+) for quality, fewer steps (20-30) for speed
pub fn image_to_video(params: ImageToVideoInput) -> FalRequest<ImageToVideoInput, Output> {
    FalRequest::new("fal-ai/ltx-video", params)
}
