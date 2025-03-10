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
pub struct HunyuanT2VRequest {
    /// The aspect ratio of the video to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// If set to true, the safety checker will be enabled.
    /// true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The LoRAs to use for the image generation. You can use any number of LoRAs
    /// and they will be merged together to generate the final image.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The number of frames to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_frames: Option<String>,
    /// By default, generations are done with 35 steps. Pro mode does 55 steps which results in higher quality videos but will take more time and cost 2x more billing units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pro_mode: Option<bool>,
    /// The prompt to generate the video from.
    /// "A stylish woman walks down a Tokyo street filled with warm glowing neon and animated city signage. She wears a black leather jacket, a long red dress, and black boots, and carries a black purse."
    pub prompt: String,
    /// The resolution of the video to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// The seed to use for generating the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HunyuanT2VResponse {
    /// The seed used for generating the video.
    pub seed: i64,
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LoraWeight {
    /// URL or the path to the LoRA weights.
    pub path: String,
    /// The scale of the LoRA weight. This is used to scale the LoRA weight
    /// before merging it with the base model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ValidationError {
    pub loc: Vec<serde_json::Value>,
    pub msg: String,
    #[serde(rename = "type")]
    pub ty: String,
}

/// Hunyuan Video LoRA Inference
///
/// Category: text-to-video
/// Machine Type: H100
pub fn hunyuan_video_lora(
    params: HunyuanT2VRequest,
) -> FalRequest<HunyuanT2VRequest, HunyuanT2VResponse> {
    FalRequest::new("fal-ai/hunyuan-video-lora", params)
}
