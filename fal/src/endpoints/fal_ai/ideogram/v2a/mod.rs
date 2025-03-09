#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod remix;
pub mod turbo;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        pub images: Vec<File>,
/// Seed used for the random number generator/// Seed used for the random number generator/// 123456

pub seed: i64
    }
    

                /// Ideogram V2
/// 
/// Generate high-quality images, posters, and logos with Ideogram V2. Features exceptional typography handling and realistic outputs optimized for commercial and creative use.
/// 
/// Category: text-to-image
/// 
/// License Type: commercial
/// 
/// Ideogram 2a - faster, more affordable model with better text accuracy. Can be used as an API directly from fal.
                pub fn v2a(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/ideogram/v2",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        pub images: Vec<File>,
/// Seed used for the random number generator/// Seed used for the random number generator/// 123456

pub seed: i64
    }
    

                /// Ideogram V2 Edit
/// 
/// Transform existing images with Ideogram V2's editing capabilities. Modify, adjust, and refine images while maintaining high fidelity and realistic outputs with precise prompt control.
/// 
/// Category: image-to-image
/// 
/// License Type: commercial
/// 
/// Ideogram 2a - faster, more affordable model with better text accuracy. Can be used as an API directly from fal.
                pub fn v2a(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/ideogram/v2/edit",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        pub images: Vec<File>,
/// Seed used for the random number generator/// Seed used for the random number generator/// 123456

pub seed: i64
    }
    

                /// Ideogram V2 Remix
/// 
/// Reimagine existing images with Ideogram V2's remix feature. Create variations and adaptations while preserving core elements and adding new creative directions through prompt guidance.
/// 
/// Category: image-to-image
/// 
/// License Type: commercial
/// 
/// Ideogram 2a - faster, more affordable model with better text accuracy. Can be used as an API directly from fal.
                pub fn v2a(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/ideogram/v2/remix",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        pub images: Vec<File>,
/// Seed used for the random number generator/// Seed used for the random number generator/// 123456

pub seed: i64
    }
    

                /// Ideogram V2 Turbo
/// 
/// Accelerated image generation with Ideogram V2 Turbo. Create high-quality visuals, posters, and logos with enhanced speed while maintaining Ideogram's signature quality.
/// 
/// Category: text-to-image
/// 
/// License Type: commercial
/// 
/// Ideogram 2a - faster, more affordable model with better text accuracy. Can be used as an API directly from fal.
                pub fn v2a(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/ideogram/v2/turbo",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        pub images: Vec<File>,
/// Seed used for the random number generator/// Seed used for the random number generator/// 123456

pub seed: i64
    }
    

                /// Ideogram V2 Turbo Edit
/// 
/// Edit images faster with Ideogram V2 Turbo. Quick modifications and adjustments while preserving the high-quality standards and realistic outputs of Ideogram.
/// 
/// Category: image-to-image
/// 
/// License Type: commercial
/// 
/// Ideogram 2a - faster, more affordable model with better text accuracy. Can be used as an API directly from fal.
                pub fn v2a(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/ideogram/v2/turbo/edit",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        pub images: Vec<File>,
/// Seed used for the random number generator/// Seed used for the random number generator/// 123456

pub seed: i64
    }
    

                /// Ideogram V2 Turbo Remix
/// 
/// Rapidly create image variations with Ideogram V2 Turbo Remix. Fast and efficient reimagining of existing images while maintaining creative control through prompt guidance.
/// 
/// Category: image-to-image
/// 
/// License Type: commercial
/// 
/// Ideogram 2a - faster, more affordable model with better text accuracy. Can be used as an API directly from fal.
                pub fn v2a(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/ideogram/v2/turbo/remix",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        pub images: Vec<File>,
/// Seed used for the random number generator/// Seed used for the random number generator/// 123456

pub seed: i64
    }
    

                /// Ideogram Upscale
/// 
/// Ideogram Upscale enhances the resolution of the reference image by up to 2X and might enhance the reference image too. Optionally refine outputs with a prompt for guided improvements.
/// 
/// Category: image-to-image
/// 
/// License Type: commercial
/// 
/// Ideogram 2a - faster, more affordable model with better text accuracy. Can be used as an API directly from fal.
                pub fn v2a(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/ideogram/upscale",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        pub images: Vec<File>,
/// Seed used for the random number generator/// Seed used for the random number generator/// 123456

pub seed: i64
    }
    

                /// Ideogram V2A
/// 
/// Generate high-quality images, posters, and logos with Ideogram V2A. Features exceptional typography handling and realistic outputs optimized for commercial and creative use.
/// 
/// Category: text-to-image
/// 
/// License Type: commercial
/// 
/// Ideogram 2a - faster, more affordable model with better text accuracy. Can be used as an API directly from fal.
                pub fn v2a(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/ideogram/v2a",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        pub images: Vec<File>,
/// Seed used for the random number generator/// Seed used for the random number generator/// 123456

pub seed: i64
    }
    

                /// Ideogram V2A Remix
/// 
/// Create variations of existing images with Ideogram V2A Remix while maintaining creative control through prompt guidance.
/// 
/// Category: image-to-image
/// 
/// License Type: commercial
/// 
/// Ideogram 2a - faster, more affordable model with better text accuracy. Can be used as an API directly from fal.
                pub fn v2a(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/ideogram/v2a/remix",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        pub images: Vec<File>,
/// Seed used for the random number generator/// Seed used for the random number generator/// 123456

pub seed: i64
    }
    

                /// Ideogram V2A Turbo
/// 
/// Accelerated image generation with Ideogram V2A Turbo. Create high-quality visuals, posters, and logos with enhanced speed while maintaining Ideogram's signature quality.
/// 
/// Category: text-to-image
/// 
/// License Type: commercial
/// 
/// Ideogram 2a - faster, more affordable model with better text accuracy. Can be used as an API directly from fal.
                pub fn v2a(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/ideogram/v2a/turbo",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        pub images: Vec<File>,
/// Seed used for the random number generator/// Seed used for the random number generator/// 123456

pub seed: i64
    }
    

                /// Ideogram V2A Turbo Remix
/// 
/// Rapidly create image variations with Ideogram V2A Turbo Remix. Fast and efficient reimagining of existing images while maintaining creative control through prompt guidance.
/// 
/// Category: image-to-image
/// 
/// License Type: commercial
/// 
/// Ideogram 2a - faster, more affordable model with better text accuracy. Can be used as an API directly from fal.
                pub fn v2a(params: TextToImageInput) -> FalRequest<TextToImageInput, Output> {
                    FalRequest::new(
                        "fal-ai/ideogram/v2a/turbo/remix",
                        params
                    )
                }
                