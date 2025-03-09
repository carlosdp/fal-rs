#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// The seed used for generating the video.
pub seed: i64,
/// The generated video/// The generated video/// {"content_type":"video/mp4","url":"https://storage.googleapis.com/falserverless/gallery/man-smiles.mp4"}

pub video: File
    }
    

                /// Hunyuan Video Image-to-Video LoRA Inference
/// 
/// Image to Video for the Hunyuan Video model using a custom trained LoRA.
/// 
/// Category: image-to-video
/// Machine Type: H100
/// 
/// 
/// Generate a video based on a prompt and an image URL.
/// This implementation downloads the image from the URL, replicates it to form a video,
/// encodes the prompt, uses video2video conditioning and sampling to produce new video latents,
/// decodes the latents to video frames, and saves the video to a temporary file.
                pub fn hunyuan_video_img2vid_lora(params: Input) -> FalRequest<Input, Output> {
                    FalRequest::new(
                        "fal-ai/hunyuan-video-img2vid-lora",
                        params
                    )
                }
                