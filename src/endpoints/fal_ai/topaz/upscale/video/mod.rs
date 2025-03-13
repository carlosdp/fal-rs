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

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoUpscaleOutput {
    /// The upscaled video file
    /// {"url":"https://v3.fal.media/files/penguin/ztj_LB4gQlW6HIfVs8zX4_upscaled.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VideoUpscaleRequest {
    /// Target FPS for frame interpolation. If set, frame interpolation will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_fps: Option<i64>,
    /// Factor to upscale the video by (e.g. 2.0 doubles width and height)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upscale_factor: Option<f64>,
    /// URL of the video to upscale
    /// "https://v3.fal.media/files/kangaroo/y5-1YTGpun17eSeggZMzX_video-1733468228.mp4"
    pub video_url: String,
}

/// Topaz Video Upscale
///
/// Category: video-to-video
///
///
///
/// Upscale videos using Topaz Video AI.
///
/// Uses Proteus v4 for upscaling and optionally Apollo v8 for frame interpolation.
/// Supports up to 8x upscaling and 120 FPS output. Frame interpolation is automatically
/// enabled when target_fps is set.
pub fn video(params: VideoUpscaleRequest) -> FalRequest<VideoUpscaleRequest, VideoUpscaleOutput> {
    FalRequest::new("fal-ai/topaz/upscale/video", params)
}
