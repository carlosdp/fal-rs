#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SAM2AutomaticSegmentationOutput {
        /// Combined segmentation mask.
pub combined_mask: Image,
/// Individual segmentation masks.
pub individual_masks: Vec<Image>
    }
    

                /// Segment Anything Model 2
/// 
/// SAM 2 is a model for segmenting images and videos in real-time.
/// 
/// Category: image-to-image
/// Machine Type: A100
                pub fn auto_segment(params: SAM2AutomaticSegmentationInput) -> FalRequest<SAM2AutomaticSegmentationInput, SAM2AutomaticSegmentationOutput> {
                    FalRequest::new(
                        "fal-ai/sam2/image",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SAM2AutomaticSegmentationOutput {
        /// Combined segmentation mask.
pub combined_mask: Image,
/// Individual segmentation masks.
pub individual_masks: Vec<Image>
    }
    

                /// Segment Anything Model 2
/// 
/// SAM 2 is a model for segmenting images and videos in real-time.
/// 
/// Category: video-to-video
/// Machine Type: A100
                pub fn auto_segment(params: SAM2AutomaticSegmentationInput) -> FalRequest<SAM2AutomaticSegmentationInput, SAM2AutomaticSegmentationOutput> {
                    FalRequest::new(
                        "fal-ai/sam2/video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SAM2AutomaticSegmentationOutput {
        /// Combined segmentation mask.
pub combined_mask: Image,
/// Individual segmentation masks.
pub individual_masks: Vec<Image>
    }
    

                /// Segment Anything Model 2
/// 
/// SAM 2 is a model for segmenting images automatically. It can return individual masks or a single mask for the entire image.
/// 
/// Category: image-to-image
/// Machine Type: A100
                pub fn auto_segment(params: SAM2AutomaticSegmentationInput) -> FalRequest<SAM2AutomaticSegmentationInput, SAM2AutomaticSegmentationOutput> {
                    FalRequest::new(
                        "fal-ai/sam2/auto-segment",
                        params
                    )
                }
                