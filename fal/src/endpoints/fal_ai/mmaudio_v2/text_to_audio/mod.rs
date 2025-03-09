#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AudioOutput {
        /// The generated audio./// The generated audio./// {"content_type":"application/octet-stream","file_name":"mmaudio_input.flac","file_size":1001342,"url":"https://storage.googleapis.com/falserverless/model_tests/video_models/mmaudio_output.flac"}

pub audio: File
    }
    

                /// MMAudio V2
/// 
/// MMAudio generates synchronized audio given video and/or text inputs. It can be combined with video models to get videos with audio.
/// 
/// Category: video-to-video
/// Machine Type: A100
                pub fn text_to_audio(params: AudioInput) -> FalRequest<AudioInput, AudioOutput> {
                    FalRequest::new(
                        "fal-ai/mmaudio-v2",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AudioOutput {
        /// The generated audio./// The generated audio./// {"content_type":"application/octet-stream","file_name":"mmaudio_input.flac","file_size":1001342,"url":"https://storage.googleapis.com/falserverless/model_tests/video_models/mmaudio_output.flac"}

pub audio: File
    }
    

                /// MMAudio V2 Text to Audio
/// 
/// MMAudio generates synchronized audio given text inputs. It can generate sounds described by a prompt.
/// 
/// Category: text-to-audio
/// Machine Type: A100
                pub fn text_to_audio(params: AudioInput) -> FalRequest<AudioInput, AudioOutput> {
                    FalRequest::new(
                        "fal-ai/mmaudio-v2/text-to-audio",
                        params
                    )
                }
                