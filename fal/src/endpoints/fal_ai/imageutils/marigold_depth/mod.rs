#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MarigoldDepthMapOutput {
        /// The depth map.
pub image: Image
    }
    

                /// Midas Depth Estimation
/// 
/// Create depth maps using Midas depth estimation.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn marigold_depth(params: MarigoldDepthMapInput) -> FalRequest<MarigoldDepthMapInput, MarigoldDepthMapOutput> {
                    FalRequest::new(
                        "fal-ai/imageutils/depth",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MarigoldDepthMapOutput {
        /// The depth map.
pub image: Image
    }
    

                /// Remove Background
/// 
/// Remove the background from an image.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn marigold_depth(params: MarigoldDepthMapInput) -> FalRequest<MarigoldDepthMapInput, MarigoldDepthMapOutput> {
                    FalRequest::new(
                        "fal-ai/imageutils/rembg",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MarigoldDepthMapOutput {
        /// The depth map.
pub image: Image
    }
    

                /// Marigold Depth Estimation
/// 
/// Create depth maps using Marigold depth estimation.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn marigold_depth(params: MarigoldDepthMapInput) -> FalRequest<MarigoldDepthMapInput, MarigoldDepthMapOutput> {
                    FalRequest::new(
                        "fal-ai/imageutils/marigold-depth",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MarigoldDepthMapOutput {
        /// The depth map.
pub image: Image
    }
    

                /// NSFW Filter
/// 
/// Predict the probability of an image being NSFW.
/// 
/// Category: vision
/// Machine Type: A6000
                pub fn marigold_depth(params: MarigoldDepthMapInput) -> FalRequest<MarigoldDepthMapInput, MarigoldDepthMapOutput> {
                    FalRequest::new(
                        "fal-ai/imageutils/nsfw",
                        params
                    )
                }
                