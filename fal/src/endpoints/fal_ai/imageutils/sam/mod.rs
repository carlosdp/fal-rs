#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SamOutput {
        /// Combined image of all detected masks
#[serde(skip_serializing_if = "Option::is_none")]
pub image: Option<Option<Image>>
    }
    

                /// Midas Depth Estimation
/// 
/// Create depth maps using Midas depth estimation.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn sam(params: SamInput) -> FalRequest<SamInput, SamOutput> {
                    FalRequest::new(
                        "fal-ai/imageutils/depth",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SamOutput {
        /// Combined image of all detected masks
#[serde(skip_serializing_if = "Option::is_none")]
pub image: Option<Option<Image>>
    }
    

                /// Remove Background
/// 
/// Remove the background from an image.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn sam(params: SamInput) -> FalRequest<SamInput, SamOutput> {
                    FalRequest::new(
                        "fal-ai/imageutils/rembg",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SamOutput {
        /// Combined image of all detected masks
#[serde(skip_serializing_if = "Option::is_none")]
pub image: Option<Option<Image>>
    }
    

                /// Marigold Depth Estimation
/// 
/// Create depth maps using Marigold depth estimation.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn sam(params: SamInput) -> FalRequest<SamInput, SamOutput> {
                    FalRequest::new(
                        "fal-ai/imageutils/marigold-depth",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SamOutput {
        /// Combined image of all detected masks
#[serde(skip_serializing_if = "Option::is_none")]
pub image: Option<Option<Image>>
    }
    

                /// NSFW Filter
/// 
/// Predict the probability of an image being NSFW.
/// 
/// Category: vision
/// Machine Type: A6000
                pub fn sam(params: SamInput) -> FalRequest<SamInput, SamOutput> {
                    FalRequest::new(
                        "fal-ai/imageutils/nsfw",
                        params
                    )
                }
                