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
    

                /// FLUX.1 [dev]
/// 
/// FLUX.1 Image-to-Image is a high-performance endpoint for the FLUX.1 [dev] model that enables rapid transformation of existing images, delivering high-quality style transfers and image modifications with the core FLUX capabilities.
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX.1 Redux [schnell], remix your images at turbo-speed with FLUX model.
                pub fn redux(params: SchnellReduxInput) -> FalRequest<SchnellReduxInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux/dev/image-to-image",
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
    

                /// FLUX.1 [dev]
/// 
/// FLUX.1 [dev] is a 12 billion parameter flow transformer that generates high-quality images from text. It is suitable for personal and commercial use.
/// 
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX.1 Redux [schnell], remix your images at turbo-speed with FLUX model.
                pub fn redux(params: SchnellReduxInput) -> FalRequest<SchnellReduxInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux/dev",
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
    

                /// FLUX.1 [schnell]
/// 
/// FLUX.1 [schnell] is a 12 billion parameter flow transformer that generates high-quality images from text in 1 to 4 steps, suitable for personal and commercial use.
/// 
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX.1 Redux [schnell], remix your images at turbo-speed with FLUX model.
                pub fn redux(params: SchnellReduxInput) -> FalRequest<SchnellReduxInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux/schnell",
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
    

                /// FLUX.1 [schnell] Redux
/// 
/// FLUX.1 [schnell] Redux is a high-performance endpoint for the FLUX.1 [schnell] model that enables rapid transformation of existing images, delivering high-quality style transfers and image modifications with the core FLUX capabilities.
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX.1 Redux [schnell], remix your images at turbo-speed with FLUX model.
                pub fn redux(params: SchnellReduxInput) -> FalRequest<SchnellReduxInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux/schnell/redux",
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
    

                /// FLUX.1 [dev] Redux
/// 
/// FLUX.1 [dev] Redux is a high-performance endpoint for the FLUX.1 [dev] model that enables rapid transformation of existing images, delivering high-quality style transfers and image modifications with the core FLUX capabilities.
/// 
/// Category: image-to-image
/// Machine Type: H100
/// License Type: commercial
/// 
/// FLUX.1 Redux [schnell], remix your images at turbo-speed with FLUX model.
                pub fn redux(params: SchnellReduxInput) -> FalRequest<SchnellReduxInput, Output> {
                    FalRequest::new(
                        "fal-ai/flux/dev/redux",
                        params
                    )
                }
                