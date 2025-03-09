#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioOutput {
    /// The generated audio./// The generated audio./// {"content_type":"application/octet-stream","file_name":"mmaudio_input.flac","file_size":1001342,"url":"https://storage.googleapis.com/falserverless/model_tests/video_models/mmaudio_output.flac"}
    pub audio: File,
}

/// MMAudio V2
///
/// Category: video-to-video
/// Machine Type: A100
pub fn text_to_audio(params: AudioInput) -> FalRequest<AudioInput, AudioOutput> {
    FalRequest::new("fal-ai/mmaudio-v2/text-to-audio", params)
}
