#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TTSOutput {
        /// The audio file containing the generated speech.
pub audio_url: AudioFile
    }
    

                /// F5 TTS
/// 
/// F5 TTS
/// 
/// Category: text-to-audio
/// Machine Type: A100
                pub fn f5_tts(params: TTSInput) -> FalRequest<TTSInput, TTSOutput> {
                    FalRequest::new(
                        "fal-ai/f5-tts",
                        params
                    )
                }
                