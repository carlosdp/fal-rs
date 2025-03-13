#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BaseInput {
    /// Aspect ratio of the output video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// The type of effect to apply to the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_type: Option<String>,
    /// Frames per second of the generated video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frames_per_second: Option<i64>,
    /// URL of the input image.
    /// "https://storage.googleapis.com/falserverless/web-examples/wan-effects/cat.jpg"
    pub image_url: String,
    /// The scale of the LoRA weight. Used to adjust effect intensity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lora_scale: Option<f64>,
    /// Number of frames to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_frames: Option<i64>,
    /// Number of inference steps for sampling. Higher values give better quality but take longer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// Random seed for reproducibility. If None, a random seed is chosen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The subject to insert into the predefined prompt template for the selected effect.
    /// "a cute kitten"
    /// "Donald Trump"
    /// "a tank"
    /// "a ceramic vase"
    pub subject: String,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct WanEffectsOutput {
    pub seed: i64,
    /// The generated video
    /// {"url":"https://storage.googleapis.com/falserverless/web-examples/wan-effects/cat_video.mp4"}
    pub video: File,
}

/// Wan Effects
///
/// Category: image-to-video
/// Machine Type: H100
pub fn wan_effects(params: BaseInput) -> FalRequest<BaseInput, WanEffectsOutput> {
    FalRequest::new("fal-ai/wan-effects", params)
}
