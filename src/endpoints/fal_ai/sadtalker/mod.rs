#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_sadtalker",
    feature = "endpoints_fal-ai_sadtalker_reference"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_sadtalker",
        feature = "endpoints_fal-ai_sadtalker_reference"
    )))
)]
pub mod reference;

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    /// The mime type of the file.
    /// "image/png"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct HTTPValidationError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Vec<Option<ValidationError>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SadTalkerInput {
    /// URL of the driven audio
    /// "https://storage.googleapis.com/falserverless/model_tests/sadtalker/deyu.wav"
    pub driven_audio_url: String,
    /// The scale of the expression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_scale: Option<f64>,
    /// The type of face enhancer to use
    /// null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_enhancer: Option<String>,
    /// The resolution of the face model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_resolution: Option<String>,
    /// The style of the pose
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose_style: Option<i64>,
    /// The type of preprocessing to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preprocess: Option<String>,
    /// URL of the source image
    /// "https://storage.googleapis.com/falserverless/model_tests/sadtalker/anime_girl.png"
    pub source_image_url: String,
    /// Whether to use still mode. Fewer head motion, works with preprocess `full`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub still_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SadTalkerOutput {
    /// URL of the generated video
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SadTalkerRefVideoInput {
    /// URL of the driven audio
    /// "https://storage.googleapis.com/falserverless/model_tests/sadtalker/deyu.wav"
    pub driven_audio_url: String,
    /// The scale of the expression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_scale: Option<f64>,
    /// The type of face enhancer to use
    /// null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_enhancer: Option<String>,
    /// The resolution of the face model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_resolution: Option<String>,
    /// The style of the pose
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose_style: Option<i64>,
    /// The type of preprocessing to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preprocess: Option<String>,
    /// URL of the reference video
    /// "https://github.com/OpenTalker/SadTalker/raw/main/examples/ref_video/WDA_AlexandriaOcasioCortez_000.mp4"
    pub reference_pose_video_url: String,
    /// URL of the source image
    /// "https://storage.googleapis.com/falserverless/model_tests/sadtalker/anime_girl.png"
    pub source_image_url: String,
    /// Whether to use still mode. Fewer head motion, works with preprocess `full`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub still_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationError {
    pub loc: Vec<serde_json::Value>,
    pub msg: String,
    #[serde(rename = "type")]
    pub ty: String,
}

/// Sad Talker
///
/// Category: image-to-video
/// Machine Type: A100
pub fn sadtalker(params: SadTalkerInput) -> FalRequest<SadTalkerInput, SadTalkerOutput> {
    FalRequest::new("fal-ai/sadtalker", params)
}
