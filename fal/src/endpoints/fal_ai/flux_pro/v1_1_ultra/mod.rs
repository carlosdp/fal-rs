#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod redux;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image files info.
pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// FLUX1.1 [pro] ultra
/// 
/// FLUX1.1 [pro] ultra is the newest version of FLUX1.1 [pro], maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
/// 
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX1.1 [pro], next generation text-to-image model, with 10x accelerated speeds.
/// 
/// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
                pub fn v1_1_ultra(params: FluxProUltraTextToImageInput) -> FalRequest<FluxProUltraTextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-pro/v1.1-ultra",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image files info.
pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// FLUX1.1 [pro] ultra Fine-tuned
/// 
/// FLUX1.1 [pro] ultra fine-tuned is the newest version of FLUX1.1 [pro] with a fine-tuned LoRA, maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
/// 
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX1.1 [pro], next generation text-to-image model, with 10x accelerated speeds.
/// 
/// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
                pub fn v1_1_ultra(params: FluxProUltraTextToImageInput) -> FalRequest<FluxProUltraTextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-pro/v1.1-ultra-finetuned",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image files info.
pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// FLUX.1 [pro] Redux
/// 
/// FLUX.1 [pro] Redux is a high-performance endpoint for the FLUX.1 [pro] model that enables rapid transformation of existing images, delivering high-quality style transfers and image modifications with the core FLUX capabilities.
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX1.1 [pro], next generation text-to-image model, with 10x accelerated speeds.
/// 
/// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
                pub fn v1_1_ultra(params: FluxProUltraTextToImageInput) -> FalRequest<FluxProUltraTextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-pro/v1/redux",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image files info.
pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// FLUX1.1 [pro] Redux
/// 
/// FLUX1.1 [pro] Redux is a high-performance endpoint for the FLUX1.1 [pro] model that enables rapid transformation of existing images, delivering high-quality style transfers and image modifications with the core FLUX capabilities.
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX1.1 [pro], next generation text-to-image model, with 10x accelerated speeds.
/// 
/// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
                pub fn v1_1_ultra(params: FluxProUltraTextToImageInput) -> FalRequest<FluxProUltraTextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-pro/v1.1/redux",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image files info.
pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// FLUX1.1 [pro] ultra Redux
/// 
/// FLUX1.1 [pro] ultra Redux is a high-performance endpoint for the FLUX1.1 [pro] model that enables rapid transformation of existing images, delivering high-quality style transfers and image modifications with the core FLUX capabilities.
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX1.1 [pro], next generation text-to-image model, with 10x accelerated speeds.
/// 
/// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
                pub fn v1_1_ultra(params: FluxProUltraTextToImageInput) -> FalRequest<FluxProUltraTextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-pro/v1.1-ultra/redux",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image files info.
pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// FLUX.1 [pro] Fill
/// 
/// FLUX.1 [pro] Fill is a high-performance endpoint for the FLUX.1 [pro] model that enables rapid transformation of existing images, delivering high-quality style transfers and image modifications with the core FLUX capabilities.
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX1.1 [pro], next generation text-to-image model, with 10x accelerated speeds.
/// 
/// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
                pub fn v1_1_ultra(params: FluxProUltraTextToImageInput) -> FalRequest<FluxProUltraTextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-pro/v1/fill",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image files info.
pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// FLUX.1 [pro] Fill Fine-tuned
/// 
/// FLUX.1 [pro] Fill Fine-tuned is a high-performance endpoint for the FLUX.1 [pro] model with a fine-tuned LoRA that enables rapid transformation of existing images, delivering high-quality style transfers and image modifications with the core FLUX capabilities.
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX1.1 [pro], next generation text-to-image model, with 10x accelerated speeds.
/// 
/// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
                pub fn v1_1_ultra(params: FluxProUltraTextToImageInput) -> FalRequest<FluxProUltraTextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-pro/v1/fill-finetuned",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image files info.
pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// FLUX.1 [pro] Canny
/// 
/// Utilize Flux.1 [pro] Controlnet to generate high-quality images with precise control over composition, style, and structure through advanced edge detection and guidance mechanisms.
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX1.1 [pro], next generation text-to-image model, with 10x accelerated speeds.
/// 
/// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
                pub fn v1_1_ultra(params: FluxProUltraTextToImageInput) -> FalRequest<FluxProUltraTextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-pro/v1/canny",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image files info.
pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// FLUX.1 [pro] Canny Fine-tuned
/// 
/// Utilize Flux.1 [pro] Controlnet with a fine-tuned LoRA to generate high-quality images with precise control over composition, style, and structure through advanced edge detection and guidance mechanisms.
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX1.1 [pro], next generation text-to-image model, with 10x accelerated speeds.
/// 
/// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
                pub fn v1_1_ultra(params: FluxProUltraTextToImageInput) -> FalRequest<FluxProUltraTextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-pro/v1/canny-finetuned",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image files info.
pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// FLUX.1 [pro] Depth
/// 
/// Generate high-quality images from depth maps using Flux.1 [pro] depth estimation model. The model produces accurate depth representations for scene understanding and 3D visualization.
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX1.1 [pro], next generation text-to-image model, with 10x accelerated speeds.
/// 
/// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
                pub fn v1_1_ultra(params: FluxProUltraTextToImageInput) -> FalRequest<FluxProUltraTextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-pro/v1/depth",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image files info.
pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// FLUX.1 [pro] Depth Fine-tuned
/// 
/// Generate high-quality images from depth maps using Flux.1 [pro] depth estimation model with a fine-tuned LoRA. The model produces accurate depth representations for scene understanding and 3D visualization.
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX1.1 [pro], next generation text-to-image model, with 10x accelerated speeds.
/// 
/// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
                pub fn v1_1_ultra(params: FluxProUltraTextToImageInput) -> FalRequest<FluxProUltraTextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-pro/v1/depth-finetuned",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image files info.
pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// FLUX1.1 [pro]
/// 
/// FLUX1.1 [pro] is an enhanced version of FLUX.1 [pro], improved image generation capabilities, delivering superior composition, detail, and artistic fidelity compared to its predecessor.
/// 
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX1.1 [pro], next generation text-to-image model, with 10x accelerated speeds.
/// 
/// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
                pub fn v1_1_ultra(params: FluxProUltraTextToImageInput) -> FalRequest<FluxProUltraTextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-pro/v1.1",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image files info.
pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// FLUX.1 [pro]
/// 
/// FLUX.1 [pro] new is an accelerated version of FLUX.1 [pro], maintaining professional-grade image quality while delivering significantly faster generation speeds.
/// 
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX1.1 [pro], next generation text-to-image model, with 10x accelerated speeds.
/// 
/// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
                pub fn v1_1_ultra(params: FluxProUltraTextToImageInput) -> FalRequest<FluxProUltraTextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-pro/new",
                        params
                    )
                }
                