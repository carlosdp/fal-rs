#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// URL to the lora configuration file.
pub config_file: File,
/// URL to the trained diffusers lora weights.
pub diffusers_lora_file: File
    }
    

                /// Train Hunyuan LoRA
/// 
/// Train Hunyuan Video lora on people, objects, characters and more!
/// 
/// Category: training
/// Machine Type: A100
/// 
/// 
/// Hunyuan Video LoRA fine-tuning endpoint.
/// 
/// This endpoint fine-tunes a LoRA model on a dataset of images.
/// 
/// To provide your own captions, you can include a text file with the same name as
/// the image file. For example, if you have an image `photo.jpg`, you can include a
/// text file `photo.txt` with the caption.
                pub fn hunyuan_video_lora_training(params: PublicInput) -> FalRequest<PublicInput, Output> {
                    FalRequest::new(
                        "fal-ai/hunyuan-video-lora-training",
                        params
                    )
                }
                