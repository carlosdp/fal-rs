#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BoundingBoxOutputWithLabels {
        /// Processed image
#[serde(skip_serializing_if = "Option::is_none")]
pub image: Option<Option<Image>>,
/// Results from the model
pub results: BoundingBoxes
    }
    

                /// Florence-2 Large
/// 
/// Florence-2 is an advanced vision foundation model that uses a prompt-based approach to handle a wide range of vision and vision-language tasks
/// 
/// Category: vision
/// Machine Type: A100
/// License Type: commercial
                pub fn object_detection(params: ImageInput) -> FalRequest<ImageInput, BoundingBoxOutputWithLabels> {
                    FalRequest::new(
                        "fal-ai/florence-2-large/caption",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BoundingBoxOutputWithLabels {
        /// Processed image
#[serde(skip_serializing_if = "Option::is_none")]
pub image: Option<Option<Image>>,
/// Results from the model
pub results: BoundingBoxes
    }
    

                /// Florence-2 Large
/// 
/// Florence-2 is an advanced vision foundation model that uses a prompt-based approach to handle a wide range of vision and vision-language tasks
/// 
/// Category: vision
/// Machine Type: A100
/// License Type: commercial
                pub fn object_detection(params: ImageInput) -> FalRequest<ImageInput, BoundingBoxOutputWithLabels> {
                    FalRequest::new(
                        "fal-ai/florence-2-large/detailed-caption",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BoundingBoxOutputWithLabels {
        /// Processed image
#[serde(skip_serializing_if = "Option::is_none")]
pub image: Option<Option<Image>>,
/// Results from the model
pub results: BoundingBoxes
    }
    

                /// Florence-2 Large
/// 
/// Florence-2 is an advanced vision foundation model that uses a prompt-based approach to handle a wide range of vision and vision-language tasks
/// 
/// Category: vision
/// Machine Type: A100
/// License Type: commercial
                pub fn object_detection(params: ImageInput) -> FalRequest<ImageInput, BoundingBoxOutputWithLabels> {
                    FalRequest::new(
                        "fal-ai/florence-2-large/more-detailed-caption",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BoundingBoxOutputWithLabels {
        /// Processed image
#[serde(skip_serializing_if = "Option::is_none")]
pub image: Option<Option<Image>>,
/// Results from the model
pub results: BoundingBoxes
    }
    

                /// Florence-2 Large
/// 
/// Florence-2 is an advanced vision foundation model that uses a prompt-based approach to handle a wide range of vision and vision-language tasks
/// 
/// Category: image-to-image
/// Machine Type: A100
/// License Type: commercial
                pub fn object_detection(params: ImageInput) -> FalRequest<ImageInput, BoundingBoxOutputWithLabels> {
                    FalRequest::new(
                        "fal-ai/florence-2-large/object-detection",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BoundingBoxOutputWithLabels {
        /// Processed image
#[serde(skip_serializing_if = "Option::is_none")]
pub image: Option<Option<Image>>,
/// Results from the model
pub results: BoundingBoxes
    }
    

                /// Florence-2 Large
/// 
/// Florence-2 is an advanced vision foundation model that uses a prompt-based approach to handle a wide range of vision and vision-language tasks
/// 
/// Category: image-to-image
/// Machine Type: A100
/// License Type: commercial
                pub fn object_detection(params: ImageInput) -> FalRequest<ImageInput, BoundingBoxOutputWithLabels> {
                    FalRequest::new(
                        "fal-ai/florence-2-large/dense-region-caption",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BoundingBoxOutputWithLabels {
        /// Processed image
#[serde(skip_serializing_if = "Option::is_none")]
pub image: Option<Option<Image>>,
/// Results from the model
pub results: BoundingBoxes
    }
    

                /// Florence-2 Large
/// 
/// Florence-2 is an advanced vision foundation model that uses a prompt-based approach to handle a wide range of vision and vision-language tasks
/// 
/// Category: image-to-image
/// Machine Type: A100
/// License Type: commercial
                pub fn object_detection(params: ImageInput) -> FalRequest<ImageInput, BoundingBoxOutputWithLabels> {
                    FalRequest::new(
                        "fal-ai/florence-2-large/region-proposal",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BoundingBoxOutputWithLabels {
        /// Processed image
#[serde(skip_serializing_if = "Option::is_none")]
pub image: Option<Option<Image>>,
/// Results from the model
pub results: BoundingBoxes
    }
    

                /// Florence-2 Large
/// 
/// Florence-2 is an advanced vision foundation model that uses a prompt-based approach to handle a wide range of vision and vision-language tasks
/// 
/// Category: image-to-image
/// Machine Type: A100
/// License Type: commercial
                pub fn object_detection(params: ImageInput) -> FalRequest<ImageInput, BoundingBoxOutputWithLabels> {
                    FalRequest::new(
                        "fal-ai/florence-2-large/caption-to-phrase-grounding",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BoundingBoxOutputWithLabels {
        /// Processed image
#[serde(skip_serializing_if = "Option::is_none")]
pub image: Option<Option<Image>>,
/// Results from the model
pub results: BoundingBoxes
    }
    

                /// Florence-2 Large
/// 
/// Florence-2 is an advanced vision foundation model that uses a prompt-based approach to handle a wide range of vision and vision-language tasks
/// 
/// Category: image-to-image
/// Machine Type: A100
/// License Type: commercial
                pub fn object_detection(params: ImageInput) -> FalRequest<ImageInput, BoundingBoxOutputWithLabels> {
                    FalRequest::new(
                        "fal-ai/florence-2-large/referring-expression-segmentation",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BoundingBoxOutputWithLabels {
        /// Processed image
#[serde(skip_serializing_if = "Option::is_none")]
pub image: Option<Option<Image>>,
/// Results from the model
pub results: BoundingBoxes
    }
    

                /// Florence-2 Large
/// 
/// Florence-2 is an advanced vision foundation model that uses a prompt-based approach to handle a wide range of vision and vision-language tasks
/// 
/// Category: image-to-image
/// Machine Type: A100
/// License Type: commercial
                pub fn object_detection(params: ImageInput) -> FalRequest<ImageInput, BoundingBoxOutputWithLabels> {
                    FalRequest::new(
                        "fal-ai/florence-2-large/region-to-segmentation",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BoundingBoxOutputWithLabels {
        /// Processed image
#[serde(skip_serializing_if = "Option::is_none")]
pub image: Option<Option<Image>>,
/// Results from the model
pub results: BoundingBoxes
    }
    

                /// Florence-2 Large
/// 
/// Florence-2 is an advanced vision foundation model that uses a prompt-based approach to handle a wide range of vision and vision-language tasks
/// 
/// Category: image-to-image
/// Machine Type: A100
/// License Type: commercial
                pub fn object_detection(params: ImageInput) -> FalRequest<ImageInput, BoundingBoxOutputWithLabels> {
                    FalRequest::new(
                        "fal-ai/florence-2-large/open-vocabulary-detection",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BoundingBoxOutputWithLabels {
        /// Processed image
#[serde(skip_serializing_if = "Option::is_none")]
pub image: Option<Option<Image>>,
/// Results from the model
pub results: BoundingBoxes
    }
    

                /// Florence-2 Large
/// 
/// Florence-2 is an advanced vision foundation model that uses a prompt-based approach to handle a wide range of vision and vision-language tasks
/// 
/// Category: vision
/// Machine Type: A100
/// License Type: commercial
                pub fn object_detection(params: ImageInput) -> FalRequest<ImageInput, BoundingBoxOutputWithLabels> {
                    FalRequest::new(
                        "fal-ai/florence-2-large/region-to-category",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BoundingBoxOutputWithLabels {
        /// Processed image
#[serde(skip_serializing_if = "Option::is_none")]
pub image: Option<Option<Image>>,
/// Results from the model
pub results: BoundingBoxes
    }
    

                /// Florence-2 Large
/// 
/// Florence-2 is an advanced vision foundation model that uses a prompt-based approach to handle a wide range of vision and vision-language tasks
/// 
/// Category: vision
/// Machine Type: A100
/// License Type: commercial
                pub fn object_detection(params: ImageInput) -> FalRequest<ImageInput, BoundingBoxOutputWithLabels> {
                    FalRequest::new(
                        "fal-ai/florence-2-large/region-to-description",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BoundingBoxOutputWithLabels {
        /// Processed image
#[serde(skip_serializing_if = "Option::is_none")]
pub image: Option<Option<Image>>,
/// Results from the model
pub results: BoundingBoxes
    }
    

                /// Florence-2 Large
/// 
/// Florence-2 is an advanced vision foundation model that uses a prompt-based approach to handle a wide range of vision and vision-language tasks
/// 
/// Category: vision
/// Machine Type: A100
/// License Type: commercial
                pub fn object_detection(params: ImageInput) -> FalRequest<ImageInput, BoundingBoxOutputWithLabels> {
                    FalRequest::new(
                        "fal-ai/florence-2-large/ocr",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BoundingBoxOutputWithLabels {
        /// Processed image
#[serde(skip_serializing_if = "Option::is_none")]
pub image: Option<Option<Image>>,
/// Results from the model
pub results: BoundingBoxes
    }
    

                /// Florence-2 Large
/// 
/// Florence-2 is an advanced vision foundation model that uses a prompt-based approach to handle a wide range of vision and vision-language tasks
/// 
/// Category: image-to-image
/// Machine Type: A100
/// License Type: commercial
                pub fn object_detection(params: ImageInput) -> FalRequest<ImageInput, BoundingBoxOutputWithLabels> {
                    FalRequest::new(
                        "fal-ai/florence-2-large/ocr-with-region",
                        params
                    )
                }
                