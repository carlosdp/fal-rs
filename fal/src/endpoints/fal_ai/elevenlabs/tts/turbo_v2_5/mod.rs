#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TTSOutput {
        /// The generated audio file/// The generated audio file/// {"url":"https://v3.fal.media/files/zebra/zJL_oRY8h5RWwjoK1w7tx_output.mp3"}

pub audio: File
    }
    

                /// ElevenLabs Audio Isolation
/// 
/// Isolate audio tracks using ElevenLabs advanced audio isolation technology.
/// 
/// Category: audio-to-audio
/// 
/// License Type: commercial
/// 
/// ElevenLabs Turbo v2.5 model for text-to-speech generation.
/// 
/// High quality with lowest latency, ideal for real-time applications.
/// Supports 32 languages while maintaining natural voice quality.
                pub fn turbo_v2_5(params: TextToSpeechRequest) -> FalRequest<TextToSpeechRequest, TTSOutput> {
                    FalRequest::new(
                        "fal-ai/elevenlabs/audio-isolation",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TTSOutput {
        /// The generated audio file/// The generated audio file/// {"url":"https://v3.fal.media/files/zebra/zJL_oRY8h5RWwjoK1w7tx_output.mp3"}

pub audio: File
    }
    

                /// ElevenLabs TTS Multilingual v2
/// 
/// Generate multilingual text-to-speech audio using ElevenLabs TTS Multilingual v2.
/// 
/// Category: text-to-audio
/// 
/// License Type: commercial
/// 
/// ElevenLabs Turbo v2.5 model for text-to-speech generation.
/// 
/// High quality with lowest latency, ideal for real-time applications.
/// Supports 32 languages while maintaining natural voice quality.
                pub fn turbo_v2_5(params: TextToSpeechRequest) -> FalRequest<TextToSpeechRequest, TTSOutput> {
                    FalRequest::new(
                        "fal-ai/elevenlabs/tts/multilingual-v2",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TTSOutput {
        /// The generated audio file/// The generated audio file/// {"url":"https://v3.fal.media/files/zebra/zJL_oRY8h5RWwjoK1w7tx_output.mp3"}

pub audio: File
    }
    

                /// ElevenLabs Sound Effects
/// 
/// Generate sound effects using ElevenLabs advanced sound effects model.
/// 
/// Category: text-to-audio
/// 
/// License Type: commercial
/// 
/// ElevenLabs Turbo v2.5 model for text-to-speech generation.
/// 
/// High quality with lowest latency, ideal for real-time applications.
/// Supports 32 languages while maintaining natural voice quality.
                pub fn turbo_v2_5(params: TextToSpeechRequest) -> FalRequest<TextToSpeechRequest, TTSOutput> {
                    FalRequest::new(
                        "fal-ai/elevenlabs/sound-effects",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TTSOutput {
        /// The generated audio file/// The generated audio file/// {"url":"https://v3.fal.media/files/zebra/zJL_oRY8h5RWwjoK1w7tx_output.mp3"}

pub audio: File
    }
    

                /// ElevenLabs TTS Turbo v2.5
/// 
/// Generate high-speed text-to-speech audio using ElevenLabs TTS Turbo v2.5.
/// 
/// Category: text-to-speech
/// 
/// License Type: commercial
/// 
/// ElevenLabs Turbo v2.5 model for text-to-speech generation.
/// 
/// High quality with lowest latency, ideal for real-time applications.
/// Supports 32 languages while maintaining natural voice quality.
                pub fn turbo_v2_5(params: TextToSpeechRequest) -> FalRequest<TextToSpeechRequest, TTSOutput> {
                    FalRequest::new(
                        "fal-ai/elevenlabs/tts/turbo-v2.5",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TTSOutput {
        /// The generated audio file/// The generated audio file/// {"url":"https://v3.fal.media/files/zebra/zJL_oRY8h5RWwjoK1w7tx_output.mp3"}

pub audio: File
    }
    

                /// ElevenLabs Speech to Text
/// 
/// Generate text from speech using ElevenLabs advanced speech-to-text model.
/// 
/// Category: speech-to-text
/// 
/// License Type: commercial
/// 
/// ElevenLabs Turbo v2.5 model for text-to-speech generation.
/// 
/// High quality with lowest latency, ideal for real-time applications.
/// Supports 32 languages while maintaining natural voice quality.
                pub fn turbo_v2_5(params: TextToSpeechRequest) -> FalRequest<TextToSpeechRequest, TTSOutput> {
                    FalRequest::new(
                        "fal-ai/elevenlabs/speech-to-text",
                        params
                    )
                }
                