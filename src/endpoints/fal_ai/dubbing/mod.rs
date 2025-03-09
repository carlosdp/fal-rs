#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputModel {
    /// The generated video with the lip sync.
    /// {"content_type":"video/mp4","file_name":"output.mp4","file_size":120000,"url":"https://v3.fal.media/files/koala/7BzEwUucbr6yuFjpcJipl_output.mp4"}
    pub video: File,
}

/// Dubbing
///
/// Category: video-to-video
///
/// License Type: commercial
pub fn dubbing(params: InputModel) -> FalRequest<InputModel, OutputModel> {
    FalRequest::new("fal-ai/dubbing", params)
}
