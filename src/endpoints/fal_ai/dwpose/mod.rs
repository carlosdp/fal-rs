#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct DWPoseOutput {
    /// The predicted pose image
    pub image: Image,
}

/// DWPose Pose Prediction
///
/// Category: image-to-image
/// Machine Type: A6000
pub fn dwpose(params: DWPoseInput) -> FalRequest<DWPoseInput, DWPoseOutput> {
    FalRequest::new("fal-ai/dwpose", params)
}
