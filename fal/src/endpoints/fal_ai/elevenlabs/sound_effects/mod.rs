#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SoundEffectOutput {
        /// The generated sound effect audio file in MP3 format/// The generated sound effect audio file in MP3 format/// {"url":"https://v3.fal.media/files/lion/WgnO-jy6WduosuG_Ibobx_sound_effect.mp3"}

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
/// ElevenLabs Sound Effects Generation.
/// 
/// Turn text into sound effects for videos, voice-overs or video games using
/// state-of-the-art sound generation technology.
                pub fn sound_effects(params: SoundEffectRequest) -> FalRequest<SoundEffectRequest, SoundEffectOutput> {
                    FalRequest::new(
                        "fal-ai/elevenlabs/audio-isolation",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SoundEffectOutput {
        /// The generated sound effect audio file in MP3 format/// The generated sound effect audio file in MP3 format/// {"url":"https://v3.fal.media/files/lion/WgnO-jy6WduosuG_Ibobx_sound_effect.mp3"}

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
/// ElevenLabs Sound Effects Generation.
/// 
/// Turn text into sound effects for videos, voice-overs or video games using
/// state-of-the-art sound generation technology.
                pub fn sound_effects(params: SoundEffectRequest) -> FalRequest<SoundEffectRequest, SoundEffectOutput> {
                    FalRequest::new(
                        "fal-ai/elevenlabs/tts/multilingual-v2",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SoundEffectOutput {
        /// The generated sound effect audio file in MP3 format/// The generated sound effect audio file in MP3 format/// {"url":"https://v3.fal.media/files/lion/WgnO-jy6WduosuG_Ibobx_sound_effect.mp3"}

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
/// ElevenLabs Sound Effects Generation.
/// 
/// Turn text into sound effects for videos, voice-overs or video games using
/// state-of-the-art sound generation technology.
                pub fn sound_effects(params: SoundEffectRequest) -> FalRequest<SoundEffectRequest, SoundEffectOutput> {
                    FalRequest::new(
                        "fal-ai/elevenlabs/sound-effects",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SoundEffectOutput {
        /// The generated sound effect audio file in MP3 format/// The generated sound effect audio file in MP3 format/// {"url":"https://v3.fal.media/files/lion/WgnO-jy6WduosuG_Ibobx_sound_effect.mp3"}

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
/// ElevenLabs Sound Effects Generation.
/// 
/// Turn text into sound effects for videos, voice-overs or video games using
/// state-of-the-art sound generation technology.
                pub fn sound_effects(params: SoundEffectRequest) -> FalRequest<SoundEffectRequest, SoundEffectOutput> {
                    FalRequest::new(
                        "fal-ai/elevenlabs/tts/turbo-v2.5",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SoundEffectOutput {
        /// The generated sound effect audio file in MP3 format/// The generated sound effect audio file in MP3 format/// {"url":"https://v3.fal.media/files/lion/WgnO-jy6WduosuG_Ibobx_sound_effect.mp3"}

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
/// ElevenLabs Sound Effects Generation.
/// 
/// Turn text into sound effects for videos, voice-overs or video games using
/// state-of-the-art sound generation technology.
                pub fn sound_effects(params: SoundEffectRequest) -> FalRequest<SoundEffectRequest, SoundEffectOutput> {
                    FalRequest::new(
                        "fal-ai/elevenlabs/speech-to-text",
                        params
                    )
                }
                