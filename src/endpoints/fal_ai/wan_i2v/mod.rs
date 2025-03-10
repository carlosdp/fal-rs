#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct File {
    /// The mime type of the file.
    /// "image/png"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// File data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>,
    /// The name of the file. It will be auto-generated if not provided.
    /// "z9RV14K95DvU.png"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// The size of the file in bytes.
    /// 4404019
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// The URL where the file can be downloaded from.
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HTTPValidationError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Vec<Option<ValidationError>>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ValidationError {
    pub loc: Vec<serde_json::Value>,
    pub msg: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WanI2VRequest {
    /// Whether to enable prompt expansion.
    /// true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_prompt_expansion: Option<bool>,
    /// If set to true, the safety checker will be enabled.
    /// true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// Frames per second of the generated video. Must be between 5 to 24.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frames_per_second: Option<i64>,
    /// URL of the input image.
    /// "https://fal.media/files/elephant/8kkhB12hEZI2kkbU8pZPA_test.jpeg"
    pub image_url: String,
    /// Number of frames to generate. Must be between 81 to 100 (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_frames: Option<i64>,
    /// Number of inference steps for sampling. Higher values give better quality but take longer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The text prompt to guide video generation.
    /// "A stylish woman walks down a Tokyo street filled with warm glowing neon and animated city signage."
    pub prompt: String,
    /// Resolution of the generated video (480p or 720p).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// Random seed for reproducibility. If None, a random seed is chosen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WanI2VResponse {
    /// The seed used for generation.
    pub seed: i64,
    /// The generated video file.
    /// {"url":"https://v3.fal.media/files/elephant/Nj4jZupkZvR7g0QkNueJZ_video-1740522225.mp4"}
    pub video: File,
}

/// Wan-2.1 Image-to-Video
///
/// Category: image-to-video
/// Machine Type: H100
///
///
/// Generate a video from an image and prompt.
pub fn wan_i2v(params: WanI2VRequest) -> FalRequest<WanI2VRequest, WanI2VResponse> {
    FalRequest::new("fal-ai/wan-i2v", params)
}
