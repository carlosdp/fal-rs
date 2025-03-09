#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    /// The mime type of the file./// The mime type of the file./// "image/png"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// File data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>,
    /// The name of the file. It will be auto-generated if not provided./// The name of the file. It will be auto-generated if not provided./// "z9RV14K95DvU.png"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// The size of the file in bytes./// The size of the file in bytes./// 4404019

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// The URL where the file can be downloaded from.
    pub url: String,
}

/// PlayAI Text-to-Speech v3
///
/// Category: text-to-speech
pub fn dialog(params: TrainingInput) -> FalRequest<TrainingInput, File> {
    FalRequest::new("fal-ai/playai/tts/v3", params)
}
