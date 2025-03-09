#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LineartOutput {
        /// Image with edges detected using the Canny algorithm
pub image: Image
    }
    

                /// Image Preprocessors
/// 
/// Depth Anything v2 preprocessor.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn lineart(params: LineartInput) -> FalRequest<LineartInput, LineartOutput> {
                    FalRequest::new(
                        "fal-ai/image-preprocessors/depth-anything/v2",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LineartOutput {
        /// Image with edges detected using the Canny algorithm
pub image: Image
    }
    

                /// Image Preprocessors
/// 
/// Holistically-Nested Edge Detection (HED) preprocessor.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn lineart(params: LineartInput) -> FalRequest<LineartInput, LineartOutput> {
                    FalRequest::new(
                        "fal-ai/image-preprocessors/hed",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LineartOutput {
        /// Image with edges detected using the Canny algorithm
pub image: Image
    }
    

                /// Image Preprocessors
/// 
/// Line art preprocessor.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn lineart(params: LineartInput) -> FalRequest<LineartInput, LineartOutput> {
                    FalRequest::new(
                        "fal-ai/image-preprocessors/lineart",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LineartOutput {
        /// Image with edges detected using the Canny algorithm
pub image: Image
    }
    

                /// Image Preprocessors
/// 
/// MiDaS depth estimation preprocessor.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn lineart(params: LineartInput) -> FalRequest<LineartInput, LineartOutput> {
                    FalRequest::new(
                        "fal-ai/image-preprocessors/midas",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LineartOutput {
        /// Image with edges detected using the Canny algorithm
pub image: Image
    }
    

                /// Image Preprocessors
/// 
/// M-LSD line segment detection preprocessor.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn lineart(params: LineartInput) -> FalRequest<LineartInput, LineartOutput> {
                    FalRequest::new(
                        "fal-ai/image-preprocessors/mlsd",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LineartOutput {
        /// Image with edges detected using the Canny algorithm
pub image: Image
    }
    

                /// Image Preprocessors
/// 
/// PIDI (Pidinet) preprocessor.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn lineart(params: LineartInput) -> FalRequest<LineartInput, LineartOutput> {
                    FalRequest::new(
                        "fal-ai/image-preprocessors/pidi",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LineartOutput {
        /// Image with edges detected using the Canny algorithm
pub image: Image
    }
    

                /// Image Preprocessors
/// 
/// Segment Anything Model (SAM) preprocessor.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn lineart(params: LineartInput) -> FalRequest<LineartInput, LineartOutput> {
                    FalRequest::new(
                        "fal-ai/image-preprocessors/sam",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LineartOutput {
        /// Image with edges detected using the Canny algorithm
pub image: Image
    }
    

                /// Image Preprocessors
/// 
/// Scribble preprocessor.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn lineart(params: LineartInput) -> FalRequest<LineartInput, LineartOutput> {
                    FalRequest::new(
                        "fal-ai/image-preprocessors/scribble",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LineartOutput {
        /// Image with edges detected using the Canny algorithm
pub image: Image
    }
    

                /// Image Preprocessors
/// 
/// TEED (Temporal Edge Enhancement Detection) preprocessor.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn lineart(params: LineartInput) -> FalRequest<LineartInput, LineartOutput> {
                    FalRequest::new(
                        "fal-ai/image-preprocessors/teed",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LineartOutput {
        /// Image with edges detected using the Canny algorithm
pub image: Image
    }
    

                /// Image Preprocessors
/// 
/// ZoeDepth preprocessor.
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn lineart(params: LineartInput) -> FalRequest<LineartInput, LineartOutput> {
                    FalRequest::new(
                        "fal-ai/image-preprocessors/zoe",
                        params
                    )
                }
                