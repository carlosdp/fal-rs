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
    pub content_type: Option<ContentTypeProperty>,
    /// The name of the file. It will be auto-generated if not provided.
    /// "z9RV14K95DvU.png"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<FileNameProperty>,
    /// The size of the file in bytes.
    /// 4404019
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<FileSizeProperty>,
    /// The URL where the file can be downloaded from.
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HTTPValidationError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Vec<Option<ValidationError>>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LipSyncInput {
    /// URL of the input audio
    /// "https://fal.media/files/lion/vyFWygmZsIZlUO4s0nr2n.wav"
    pub audio_url: String,
    /// The model to use for lipsyncing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Lipsync mode when audio and video durations are out of sync.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<String>,
    /// URL of the input video
    /// "https://fal.media/files/koala/8teUPbRRMtAUTORDvqy0l.mp4"
    pub video_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LipSyncOutput {
    /// The generated video
    /// {"url":"https://v3.fal.media/files/rabbit/6gJV-z7RJsF0AxkZHkdgJ_output.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ValidationError {
    pub loc: Vec<serde_json::Value>,
    pub msg: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum ContentTypeProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum FileNameProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum FileSizeProperty {
    #[default]
    Integer(i64),
    Null(serde_json::Value),
}

/// sync.so -- lipsync 1.9.0-beta
///
/// Category: video-to-video
///
/// License Type: commercial
pub fn sync_lipsync(params: LipSyncInput) -> FalRequest<LipSyncInput, LipSyncOutput> {
    FalRequest::new("fal-ai/sync-lipsync", params)
}
