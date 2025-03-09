#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoseTransferOutput {
    /// Whether the image contains NSFW concepts.
    pub has_nsfw_concepts: bool,
    /// The output image./// The output image./// {"content_type":"image/jpeg","height":1024,"url":"https://fal.media/files/tiger/y6ZwaYdP9Q92FnsJcSbYz.png","width":768}
    pub image: Image,
    /// The seed for the inference.
    pub seed: i64,
}

/// Leffa Virtual TryOn
///
/// Category: image-to-image
/// Machine Type: A100
/// License Type: commercial
pub fn pose_transfer(
    params: PoseTransferInput,
) -> FalRequest<PoseTransferInput, PoseTransferOutput> {
    FalRequest::new("fal-ai/leffa/pose-transfer", params)
}
