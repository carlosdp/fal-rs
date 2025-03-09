#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TranscriptionOutput {
        /// Detected or specified language code
pub language_code: String,
/// Confidence in language detection
pub language_probability: f64,
/// The full transcribed text
pub text: String,
/// Word-level transcription details
pub words: Vec<TranscriptionWord>
    }
    

                /// ElevenLabs Audio Isolation
/// 
/// Isolate audio tracks using ElevenLabs advanced audio isolation technology.
/// 
/// Category: audio-to-audio
/// 
/// License Type: commercial
/// 
/// ElevenLabs Speech to Text: Transcribe audio with high accuracy.
/// 
/// Transforms spoken audio into text with word-level timestamps and
/// optional speaker identification. Supports 99 languages with state-of-the-art accuracy.
                pub fn speech_to_text(params: SpeechToTextRequest) -> FalRequest<SpeechToTextRequest, TranscriptionOutput> {
                    FalRequest::new(
                        "fal-ai/elevenlabs/audio-isolation",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TranscriptionOutput {
        /// Detected or specified language code
pub language_code: String,
/// Confidence in language detection
pub language_probability: f64,
/// The full transcribed text
pub text: String,
/// Word-level transcription details
pub words: Vec<TranscriptionWord>
    }
    

                /// ElevenLabs TTS Multilingual v2
/// 
/// Generate multilingual text-to-speech audio using ElevenLabs TTS Multilingual v2.
/// 
/// Category: text-to-audio
/// 
/// License Type: commercial
/// 
/// ElevenLabs Speech to Text: Transcribe audio with high accuracy.
/// 
/// Transforms spoken audio into text with word-level timestamps and
/// optional speaker identification. Supports 99 languages with state-of-the-art accuracy.
                pub fn speech_to_text(params: SpeechToTextRequest) -> FalRequest<SpeechToTextRequest, TranscriptionOutput> {
                    FalRequest::new(
                        "fal-ai/elevenlabs/tts/multilingual-v2",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TranscriptionOutput {
        /// Detected or specified language code
pub language_code: String,
/// Confidence in language detection
pub language_probability: f64,
/// The full transcribed text
pub text: String,
/// Word-level transcription details
pub words: Vec<TranscriptionWord>
    }
    

                /// ElevenLabs Sound Effects
/// 
/// Generate sound effects using ElevenLabs advanced sound effects model.
/// 
/// Category: text-to-audio
/// 
/// License Type: commercial
/// 
/// ElevenLabs Speech to Text: Transcribe audio with high accuracy.
/// 
/// Transforms spoken audio into text with word-level timestamps and
/// optional speaker identification. Supports 99 languages with state-of-the-art accuracy.
                pub fn speech_to_text(params: SpeechToTextRequest) -> FalRequest<SpeechToTextRequest, TranscriptionOutput> {
                    FalRequest::new(
                        "fal-ai/elevenlabs/sound-effects",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TranscriptionOutput {
        /// Detected or specified language code
pub language_code: String,
/// Confidence in language detection
pub language_probability: f64,
/// The full transcribed text
pub text: String,
/// Word-level transcription details
pub words: Vec<TranscriptionWord>
    }
    

                /// ElevenLabs TTS Turbo v2.5
/// 
/// Generate high-speed text-to-speech audio using ElevenLabs TTS Turbo v2.5.
/// 
/// Category: text-to-speech
/// 
/// License Type: commercial
/// 
/// ElevenLabs Speech to Text: Transcribe audio with high accuracy.
/// 
/// Transforms spoken audio into text with word-level timestamps and
/// optional speaker identification. Supports 99 languages with state-of-the-art accuracy.
                pub fn speech_to_text(params: SpeechToTextRequest) -> FalRequest<SpeechToTextRequest, TranscriptionOutput> {
                    FalRequest::new(
                        "fal-ai/elevenlabs/tts/turbo-v2.5",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TranscriptionOutput {
        /// Detected or specified language code
pub language_code: String,
/// Confidence in language detection
pub language_probability: f64,
/// The full transcribed text
pub text: String,
/// Word-level transcription details
pub words: Vec<TranscriptionWord>
    }
    

                /// ElevenLabs Speech to Text
/// 
/// Generate text from speech using ElevenLabs advanced speech-to-text model.
/// 
/// Category: speech-to-text
/// 
/// License Type: commercial
/// 
/// ElevenLabs Speech to Text: Transcribe audio with high accuracy.
/// 
/// Transforms spoken audio into text with word-level timestamps and
/// optional speaker identification. Supports 99 languages with state-of-the-art accuracy.
                pub fn speech_to_text(params: SpeechToTextRequest) -> FalRequest<SpeechToTextRequest, TranscriptionOutput> {
                    FalRequest::new(
                        "fal-ai/elevenlabs/speech-to-text",
                        params
                    )
                }
                