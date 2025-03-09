#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
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
    

                /// FLUX.1 [dev] with Controlnets and Loras
/// 
/// A versatile endpoint for the FLUX.1 [dev] model that supports multiple AI extensions including LoRA, ControlNet conditioning, and IP-Adapter integration, enabling comprehensive control over image generation through various guidance methods.
/// 
/// Category: text-to-image
/// Machine Type: A100
/// License Type: commercial
/// 
/// FLUX.1 [dev], next generation text-to-image model.
                pub fn rf_inversion(params: RFInversionInput) -> FalRequest<RFInversionInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-general",
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
    

                /// FLUX.1 [dev] with Controlnets and Loras
/// 
/// FLUX General Inpainting is a versatile endpoint that enables precise image editing and completion, supporting multiple AI extensions including LoRA, ControlNet, and IP-Adapter for enhanced control over inpainting results and sophisticated image modifications.
/// 
/// Category: image-to-image
/// Machine Type: A100
/// License Type: commercial
/// 
/// FLUX.1 [dev], next generation text-to-image model.
                pub fn rf_inversion(params: RFInversionInput) -> FalRequest<RFInversionInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-general/inpainting",
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
    

                /// FLUX.1 [dev] with Controlnets and Loras
/// 
/// FLUX General Image-to-Image is a versatile endpoint that transforms existing images with support for LoRA, ControlNet, and IP-Adapter extensions, enabling precise control over style transfer, modifications, and artistic variations through multiple guidance methods.
/// 
/// Category: image-to-image
/// Machine Type: A100
/// License Type: commercial
/// 
/// FLUX.1 [dev], next generation text-to-image model.
                pub fn rf_inversion(params: RFInversionInput) -> FalRequest<RFInversionInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-general/image-to-image",
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
    

                /// FLUX.1 [dev] with Controlnets and Loras
/// 
/// A specialized FLUX endpoint combining differential diffusion control with LoRA, ControlNet, and IP-Adapter support, enabling precise, region-specific image transformations through customizable change maps.
/// 
/// Category: image-to-image
/// Machine Type: A100
/// License Type: commercial
/// 
/// FLUX.1 [dev], next generation text-to-image model.
                pub fn rf_inversion(params: RFInversionInput) -> FalRequest<RFInversionInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-general/differential-diffusion",
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
    

                /// FLUX.1 [dev] with Controlnets and Loras
/// 
/// A general purpose endpoint for the FLUX.1 [dev] model, implementing the RF-Inversion pipeline. This can be used to edit a reference image based on a prompt.
/// 
/// Category: image-to-image
/// Machine Type: A100
/// License Type: commercial
/// 
/// FLUX.1 [dev], next generation text-to-image model.
                pub fn rf_inversion(params: RFInversionInput) -> FalRequest<RFInversionInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux-general/rf-inversion",
                        params
                    )
                }
                