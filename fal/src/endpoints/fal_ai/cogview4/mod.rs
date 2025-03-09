#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ImageOutput {
        /// Whether the generated images contain NSFW concepts.
pub has_nsfw_concepts: Vec<bool>,
/// The generated images/// The generated images/// [{"content_type":"image/jpeg","height":768,"url":"https://v3.fal.media/files/tiger/rN6_PpE-o8QlSecqFku6h.jpeg","width":1024}]

pub images: Vec<Image>,
/// The prompt used for generating the image.
pub prompt: String,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// CogView4
/// 
/// Generate high quality images from text prompts using CogView4. Longer text prompts will result in better quality images.
/// 
/// Category: text-to-image
/// Machine Type: H100
                pub fn cogview4(params: TextToImageInput) -> FalRequest<TextToImageInput, ImageOutput> {
                    FalRequest::new(
                        "fal-ai/cogview4",
                        params
                    )
                }
                