#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OutputModel {
        /// Generated video prompt/// Generated video prompt/// "A futuristic city glows softly at dusk, captured with smooth gimbal movements and a slow burn pacing, enhanced by subtle holographic overlays."

pub prompt: String
    }
    

                /// Video Prompt Generator
/// 
/// Generate video prompts using a variety of techniques including camera direction, style, pacing, special effects and more.
/// 
/// Category: llm
                pub fn video_prompt_generator(params: InputModel) -> FalRequest<InputModel, OutputModel> {
                    FalRequest::new(
                        "fal-ai/video-prompt-generator",
                        params
                    )
                }
                