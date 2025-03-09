#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod image_to_video;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ray2T2VOutput {
        /// The generated video/// The generated video/// {"url":"https://v3.fal.media/files/penguin/Om3xjcOwiSCJwrXs7DUi__output.mp4"}

pub video: File
    }
    

                /// Luma Ray 2
/// 
/// Ray2 is a large-scale video generative model capable of creating realistic visuals with natural, coherent motion.
/// 
/// Category: text-to-video
/// Machine Type: A100
/// 
/// 
/// Luma's state of the art Ray2 model for text-to-video generation.
/// 
/// Large-scale video generative model capable of creating realistic visuals with natural, coherent
/// motion. It has strong understanding of text instructions and can take image and video as input.
                pub fn ray_2(params: Ray2TextToVideoRequest) -> FalRequest<Ray2TextToVideoRequest, Ray2T2VOutput> {
                    FalRequest::new(
                        "fal-ai/luma-dream-machine/ray-2",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ray2T2VOutput {
        /// The generated video/// The generated video/// {"url":"https://v3.fal.media/files/penguin/Om3xjcOwiSCJwrXs7DUi__output.mp4"}

pub video: File
    }
    

                /// Luma Ray 2 (Image to Video)
/// 
/// Ray2 is a large-scale video generative model capable of creating realistic visuals with natural, coherent motion.
/// 
/// Category: image-to-video
/// Machine Type: A100
/// 
/// 
/// Luma's state of the art Ray2 model for text-to-video generation.
/// 
/// Large-scale video generative model capable of creating realistic visuals with natural, coherent
/// motion. It has strong understanding of text instructions and can take image and video as input.
                pub fn ray_2(params: Ray2TextToVideoRequest) -> FalRequest<Ray2TextToVideoRequest, Ray2T2VOutput> {
                    FalRequest::new(
                        "fal-ai/luma-dream-machine/ray-2/image-to-video",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ray2T2VOutput {
        /// The generated video/// The generated video/// {"url":"https://v3.fal.media/files/penguin/Om3xjcOwiSCJwrXs7DUi__output.mp4"}

pub video: File
    }
    

                /// Luma Dream Machine
/// 
/// Generate video clips from your prompts using Luma Dream Machine v1.5
/// 
/// Category: text-to-video
/// Machine Type: A100
/// 
/// 
/// Luma's state of the art Ray2 model for text-to-video generation.
/// 
/// Large-scale video generative model capable of creating realistic visuals with natural, coherent
/// motion. It has strong understanding of text instructions and can take image and video as input.
                pub fn ray_2(params: Ray2TextToVideoRequest) -> FalRequest<Ray2TextToVideoRequest, Ray2T2VOutput> {
                    FalRequest::new(
                        "fal-ai/luma-dream-machine",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ray2T2VOutput {
        /// The generated video/// The generated video/// {"url":"https://v3.fal.media/files/penguin/Om3xjcOwiSCJwrXs7DUi__output.mp4"}

pub video: File
    }
    

                /// Luma Dream Machine
/// 
/// Generate video clips from your images using Luma Dream Machine v1.5
/// 
/// Category: image-to-video
/// Machine Type: A100
/// 
/// 
/// Luma's state of the art Ray2 model for text-to-video generation.
/// 
/// Large-scale video generative model capable of creating realistic visuals with natural, coherent
/// motion. It has strong understanding of text instructions and can take image and video as input.
                pub fn ray_2(params: Ray2TextToVideoRequest) -> FalRequest<Ray2TextToVideoRequest, Ray2T2VOutput> {
                    FalRequest::new(
                        "fal-ai/luma-dream-machine/image-to-video",
                        params
                    )
                }
                