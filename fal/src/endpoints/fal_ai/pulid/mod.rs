#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OutputModel {
        /// List of generated images
pub images: Vec<Image>,
/// Random seed used for reproducibility
pub seed: i64
    }
    

                /// PuLID
/// 
/// Tuning-free ID customization.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn pulid(params: InputModel) -> FalRequest<InputModel, OutputModel> {
                    FalRequest::new(
                        "fal-ai/pulid",
                        params
                    )
                }
                