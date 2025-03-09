#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod image_to_video;
pub mod ray_2;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct T2VOutput {
        /// The generated video/// The generated video/// {"url":"https://v2.fal.media/files/807e842c734f4127a36de9262a2d292c_output.mp4"}

pub video: File
    }
    

                /// Luma Ray 2
/// 
/// Ray2 is a large-scale video generative model capable of creating realistic visuals with natural, coherent motion.
/// 
/// Category: text-to-video
/// Machine Type: A100
                pub fn luma_dream_machine(params: TextToVideoRequest) -> FalRequest<TextToVideoRequest, T2VOutput> {
                    FalRequest::new(
                        "fal-ai/luma-dream-machine/ray-2",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct T2VOutput {
        /// The generated video/// The generated video/// {"url":"https://v2.fal.media/files/807e842c734f4127a36de9262a2d292c_output.mp4"}

pub video: File
    }
    

                /// Luma Ray 2 (Image to Video)
/// 
/// Ray2 is a large-scale video generative model capable of creating realistic visuals with natural, coherent motion.
/// 
/// Category: image-to-video
/// Machine Type: A100
                pub fn luma_dream_machine(params: TextToVideoRequest) -> FalRequest<TextToVideoRequest, T2VOutput> {
                    FalRequest::new(
                        "fal-ai/luma-dream-machine/ray-2/image-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct T2VOutput {
        /// The generated video/// The generated video/// {"url":"https://v2.fal.media/files/807e842c734f4127a36de9262a2d292c_output.mp4"}

pub video: File
    }
    

                /// Luma Dream Machine
/// 
/// Generate video clips from your prompts using Luma Dream Machine v1.5
/// 
/// Category: text-to-video
/// Machine Type: A100
                pub fn luma_dream_machine(params: TextToVideoRequest) -> FalRequest<TextToVideoRequest, T2VOutput> {
                    FalRequest::new(
                        "fal-ai/luma-dream-machine",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct T2VOutput {
        /// The generated video/// The generated video/// {"url":"https://v2.fal.media/files/807e842c734f4127a36de9262a2d292c_output.mp4"}

pub video: File
    }
    

                /// Luma Dream Machine
/// 
/// Generate video clips from your images using Luma Dream Machine v1.5
/// 
/// Category: image-to-video
/// Machine Type: A100
                pub fn luma_dream_machine(params: TextToVideoRequest) -> FalRequest<TextToVideoRequest, T2VOutput> {
                    FalRequest::new(
                        "fal-ai/luma-dream-machine/image-to-video",
                        params
                    )
                }
                