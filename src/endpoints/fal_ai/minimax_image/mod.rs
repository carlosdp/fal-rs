#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MiniMaxTextToImageOutput {
    /// Generated images
    pub images: Vec<File>,
}

/// MiniMax (Hailuo AI) Text to Image
///
/// Category: text-to-image
///
///
///
/// Generate images from text prompt using MiniMax API.
pub fn minimax_image(
    params: MiniMaxTextToImageRequest,
) -> FalRequest<MiniMaxTextToImageRequest, MiniMaxTextToImageOutput> {
    FalRequest::new("fal-ai/minimax-image", params)
}
