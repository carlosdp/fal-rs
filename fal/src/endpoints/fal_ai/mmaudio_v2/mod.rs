#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;

pub mod text_to_audio;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// The generated video with the lip sync./// The generated video with the lip sync./// {"content_type":"application/octet-stream","file_name":"mmaudio_input.mp4","file_size":1001342,"url":"https://storage.googleapis.com/falserverless/model_tests/video_models/mmaudio_output.mp4"}

pub video: File
    }
    

                /// MMAudio V2
/// 
/// MMAudio generates synchronized audio given video and/or text inputs. It can be combined with video models to get videos with audio.
/// 
/// Category: video-to-video
/// Machine Type: A100
                pub fn mmaudio_v2(params: BaseInput) -> FalRequest<BaseInput, Output> {
                    FalRequest::new(
                        "fal-ai/mmaudio-v2",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// The generated video with the lip sync./// The generated video with the lip sync./// {"content_type":"application/octet-stream","file_name":"mmaudio_input.mp4","file_size":1001342,"url":"https://storage.googleapis.com/falserverless/model_tests/video_models/mmaudio_output.mp4"}

pub video: File
    }
    

                /// MMAudio V2 Text to Audio
/// 
/// MMAudio generates synchronized audio given text inputs. It can generate sounds described by a prompt.
/// 
/// Category: text-to-audio
/// Machine Type: A100
                pub fn mmaudio_v2(params: BaseInput) -> FalRequest<BaseInput, Output> {
                    FalRequest::new(
                        "fal-ai/mmaudio-v2/text-to-audio",
                        params
                    )
                }
                