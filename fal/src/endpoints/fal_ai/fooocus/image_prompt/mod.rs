#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FooocusOutput {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image file info.
pub images: Vec<Image>,
/// The time taken for the generation process.
pub timings: Timings
    }
    

                /// Fooocus
/// 
/// Default parameters with automated optimizations and quality improvements.
/// 
/// Category: text-to-image
/// Machine Type: A100
                pub fn image_prompt(params: FooocusImagePromptInput) -> FalRequest<FooocusImagePromptInput, FooocusOutput> {
                    FalRequest::new(
                        "fal-ai/fooocus",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FooocusOutput {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image file info.
pub images: Vec<Image>,
/// The time taken for the generation process.
pub timings: Timings
    }
    

                /// Fooocus Upscale or Vary
/// 
/// Default parameters with automated optimizations and quality improvements.
/// 
/// Category: text-to-image
/// Machine Type: A100
                pub fn image_prompt(params: FooocusImagePromptInput) -> FalRequest<FooocusImagePromptInput, FooocusOutput> {
                    FalRequest::new(
                        "fal-ai/fooocus/upscale-or-vary",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FooocusOutput {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image file info.
pub images: Vec<Image>,
/// The time taken for the generation process.
pub timings: Timings
    }
    

                /// Fooocus Image Prompt
/// 
/// Default parameters with automated optimizations and quality improvements.
/// 
/// Category: text-to-image
/// Machine Type: A100
                pub fn image_prompt(params: FooocusImagePromptInput) -> FalRequest<FooocusImagePromptInput, FooocusOutput> {
                    FalRequest::new(
                        "fal-ai/fooocus/image-prompt",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FooocusOutput {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated image file info.
pub images: Vec<Image>,
/// The time taken for the generation process.
pub timings: Timings
    }
    

                /// Fooocus Inpainting
/// 
/// Default parameters with automated optimizations and quality improvements.
/// 
/// Category: text-to-image
/// Machine Type: A100
                pub fn image_prompt(params: FooocusImagePromptInput) -> FalRequest<FooocusImagePromptInput, FooocusOutput> {
                    FalRequest::new(
                        "fal-ai/fooocus/inpaint",
                        params
                    )
                }
                