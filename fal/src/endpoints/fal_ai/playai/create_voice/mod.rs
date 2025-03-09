#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateVoiceOutput {
        /// The S3 URI of the cloned voice.
pub voice: String
    }
    

                /// PlayAI Text-to-Speech v3
/// 
/// Blazing-fast text-to-speech. Generate audio with improved emotional tones and extensive multilingual support. Ideal for high-volume processing and efficient workflows.
/// 
/// Category: text-to-speech
                pub fn create_voice(params: CreateVoiceInput) -> FalRequest<CreateVoiceInput, CreateVoiceOutput> {
                    FalRequest::new(
                        "fal-ai/playai/tts/v3",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateVoiceOutput {
        /// The S3 URI of the cloned voice.
pub voice: String
    }
    

                /// PlayAI Text-to-Speech Dialog
/// 
/// Generate natural-sounding multi-speaker dialogues. Perfect for expressive outputs, storytelling, games, animations, and interactive media.
/// 
/// Category: text-to-speech
                pub fn create_voice(params: CreateVoiceInput) -> FalRequest<CreateVoiceInput, CreateVoiceOutput> {
                    FalRequest::new(
                        "fal-ai/playai/tts/dialog",
                        params
                    )
                }
                