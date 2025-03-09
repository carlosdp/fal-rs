#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct InsightfaceOutput {
    /// Bounding box of the face./// Bounding box of the face./// [0,0,100,100]
    pub bbox: Vec<f64>,
    /// Confidence score of the detection./// Confidence score of the detection./// 0.9
    pub det_score: f64,
    /// Embedding of the face./// Embedding of the face./// ""
    pub embedding_file: File,
    /// faces detected sorted by size
    pub faces: Vec<FaceDetection>,
    /// Keypoints of the face./// Keypoints of the face.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kps: Option<Vec<Option<Vec<Option<f64>>>>>,
    /// Keypoints of the face on the image./// Keypoints of the face on the image.
    pub kps_image: Image,
    /// Either M or F if available./// Either M or F if available./// "M"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
}

/// Image Preprocessors
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn insightface(params: InsightfaceInput) -> FalRequest<InsightfaceInput, InsightfaceOutput> {
    FalRequest::new("fal-ai/workflowutils/insightface", params)
}
