#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MiniMaxTextToImageOutput {
        /// Generated images
pub images: Vec<File>
    }
    

                /// MiniMax (Hailuo AI) Text to Image
/// 
/// Generate high quality images from text prompts using MiniMax. Longer text prompts will result in better quality images.
/// 
/// Category: text-to-image
/// 
/// 
/// 
/// Generate images from text prompt using MiniMax API.
                pub fn minimax_image(params: MiniMaxTextToImageRequest) -> FalRequest<MiniMaxTextToImageRequest, MiniMaxTextToImageOutput> {
                    FalRequest::new(
                        "fal-ai/minimax-image",
                        params
                    )
                }
                