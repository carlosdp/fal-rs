#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DWPoseOutput {
        /// The predicted pose image
pub image: Image
    }
    

                /// DWPose Pose Prediction
/// 
/// Predict poses.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn dwpose(params: DWPoseInput) -> FalRequest<DWPoseInput, DWPoseOutput> {
                    FalRequest::new(
                        "fal-ai/dwpose",
                        params
                    )
                }
                