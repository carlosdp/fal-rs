#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct WhisperOutput {
        /// Timestamp chunks of the audio file
pub chunks: Vec<WhisperChunk>,
/// Transcription of the audio file
pub text: String
    }
    

                /// Wizper (Whisper v3 -- fal.ai edition)
/// 
/// [Experimental] Whisper v3 Large -- but optimized by our inference wizards. Same WER, double the performance!
/// 
/// Category: speech-to-text
/// Machine Type: A100
/// 
/// 
/// Transcribe an audio file using the Whisper model.
                pub fn wizper(params: WhisperInput) -> FalRequest<WhisperInput, WhisperOutput> {
                    FalRequest::new(
                        "fal-ai/wizper",
                        params
                    )
                }
                