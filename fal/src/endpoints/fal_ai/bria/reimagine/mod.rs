#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReimagineOutput {
        /// The generated images/// The generated images/// [{"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_reimagine_output.png"}]

pub images: Vec<Image>,
/// Seed value used for generation.
pub seed: i64
    }
    

                /// Bria Text-to-Image Base
/// 
/// Bria's Text-to-Image model, trained exclusively on licensed data for safe and risk-free commercial use. Available also as source code and weights. For access to weights: https://bria.ai/contact-us
/// 
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
                pub fn reimagine(params: ReimagineInput) -> FalRequest<ReimagineInput, ReimagineOutput> {
                    FalRequest::new(
                        "fal-ai/bria/text-to-image/base",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReimagineOutput {
        /// The generated images/// The generated images/// [{"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_reimagine_output.png"}]

pub images: Vec<Image>,
/// Seed value used for generation.
pub seed: i64
    }
    

                /// Bria Text-to-Image Fast
/// 
/// Bria's Text-to-Image model with perfect harmony of latency and quality. Trained exclusively on licensed data for safe and risk-free commercial use. Available also as source code and weights. For access to weights: https://bria.ai/contact-us
/// 
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
                pub fn reimagine(params: ReimagineInput) -> FalRequest<ReimagineInput, ReimagineOutput> {
                    FalRequest::new(
                        "fal-ai/bria/text-to-image/fast",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReimagineOutput {
        /// The generated images/// The generated images/// [{"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_reimagine_output.png"}]

pub images: Vec<Image>,
/// Seed value used for generation.
pub seed: i64
    }
    

                /// Bria Text-to-Image HD
/// 
/// Bria's Text-to-Image model for HD images. Trained exclusively on licensed data for safe and risk-free commercial use. Available also as source code and weights. For access to weights: https://bria.ai/contact-us
/// 
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
                pub fn reimagine(params: ReimagineInput) -> FalRequest<ReimagineInput, ReimagineOutput> {
                    FalRequest::new(
                        "fal-ai/bria/text-to-image/hd",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReimagineOutput {
        /// The generated images/// The generated images/// [{"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_reimagine_output.png"}]

pub images: Vec<Image>,
/// Seed value used for generation.
pub seed: i64
    }
    

                /// Bria Eraser
/// 
/// Bria Eraser enables precise removal of unwanted objects from images while maintaining high-quality outputs. Trained exclusively on licensed data for safe and risk-free commercial use. Access the model's source code and weights: https://bria.ai/contact-us
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
                pub fn reimagine(params: ReimagineInput) -> FalRequest<ReimagineInput, ReimagineOutput> {
                    FalRequest::new(
                        "fal-ai/bria/eraser",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReimagineOutput {
        /// The generated images/// The generated images/// [{"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_reimagine_output.png"}]

pub images: Vec<Image>,
/// Seed value used for generation.
pub seed: i64
    }
    

                /// Bria Product Shot
/// 
/// Place any product in any scenery with just a prompt or reference image while maintaining high integrity of the product. Trained exclusively on licensed data for safe and risk-free commercial use and optimized for eCommerce.
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
                pub fn reimagine(params: ReimagineInput) -> FalRequest<ReimagineInput, ReimagineOutput> {
                    FalRequest::new(
                        "fal-ai/bria/product-shot",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReimagineOutput {
        /// The generated images/// The generated images/// [{"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_reimagine_output.png"}]

pub images: Vec<Image>,
/// Seed value used for generation.
pub seed: i64
    }
    

                /// Bria Background Replace
/// 
/// Bria Background Replace allows for efficient swapping of backgrounds in images via text prompts or reference image, delivering realistic and polished results. Trained exclusively on licensed data for safe and risk-free commercial use
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
                pub fn reimagine(params: ReimagineInput) -> FalRequest<ReimagineInput, ReimagineOutput> {
                    FalRequest::new(
                        "fal-ai/bria/background/replace",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReimagineOutput {
        /// The generated images/// The generated images/// [{"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_reimagine_output.png"}]

pub images: Vec<Image>,
/// Seed value used for generation.
pub seed: i64
    }
    

                /// Bria GenFill
/// 
/// Bria GenFill enables high-quality object addition or visual transformation. Trained exclusively on licensed data for safe and risk-free commercial use. Access the model's source code and weights: https://bria.ai/contact-us
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
                pub fn reimagine(params: ReimagineInput) -> FalRequest<ReimagineInput, ReimagineOutput> {
                    FalRequest::new(
                        "fal-ai/bria/genfill",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReimagineOutput {
        /// The generated images/// The generated images/// [{"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_reimagine_output.png"}]

pub images: Vec<Image>,
/// Seed value used for generation.
pub seed: i64
    }
    

                /// Bria Expand Image
/// 
/// Bria Expand expands images beyond their borders in high quality. Trained exclusively on licensed data for safe and risk-free commercial use. Access the model's source code and weights: https://bria.ai/contact-us
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
                pub fn reimagine(params: ReimagineInput) -> FalRequest<ReimagineInput, ReimagineOutput> {
                    FalRequest::new(
                        "fal-ai/bria/expand",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReimagineOutput {
        /// The generated images/// The generated images/// [{"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_reimagine_output.png"}]

pub images: Vec<Image>,
/// Seed value used for generation.
pub seed: i64
    }
    

                /// Bria RMBG 2.0
/// 
/// Bria RMBG 2.0 enables seamless removal of backgrounds from images, ideal for professional editing tasks. Trained exclusively on licensed data for safe and risk-free commercial use. Model weights for commercial use are available here: https://share-eu1.hsforms.com/2GLpEVQqJTI2Lj7AMYwgfIwf4e04?utm_campaign=RMBG%202.0&utm_source=RMBG%20image%20and%20video%20page&utm_medium=button&utm_content=rmbg%20image%20pricing%20form
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
                pub fn reimagine(params: ReimagineInput) -> FalRequest<ReimagineInput, ReimagineOutput> {
                    FalRequest::new(
                        "fal-ai/bria/background/remove",
                        params
                    )
                }
                