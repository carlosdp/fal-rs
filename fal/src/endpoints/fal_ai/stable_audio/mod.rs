#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// The generated audio clip
pub audio_file: File
    }
    

                /// Stable Audio Open
/// 
/// Open source text-to-audio model.
/// 
/// Category: text-to-audio
/// Machine Type: A100
                pub fn stable_audio(params: Input) -> FalRequest<Input, Output> {
                    FalRequest::new(
                        "fal-ai/stable-audio",
                        params
                    )
                }
                