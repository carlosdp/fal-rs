#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectReferenceOutput {
    /// The generated video/// The generated video/// {"url":"https://fal.media/files/rabbit/pONKqOnY7z6GlF6oDESvR_output.mp4"}
    pub video: File,
}

/// MiniMax (Hailuo AI) Video 01 Live
///
/// Category: text-to-video
///
///
///
/// Hailuo S2V-01 API: Text-to-video generation with subject reference image for consistent character appearance
pub fn video_01_subject_reference(
    params: SubjectReferenceRequest,
) -> FalRequest<SubjectReferenceRequest, SubjectReferenceOutput> {
    FalRequest::new("fal-ai/minimax/video-01-live", params)
}
