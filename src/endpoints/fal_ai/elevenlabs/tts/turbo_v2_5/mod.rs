#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AudioIsolationRequest {
    /// URL of the audio file to isolate voice from
    /// "https://v3.fal.media/files/zebra/zJL_oRY8h5RWwjoK1w7tx_output.mp3"
    pub audio_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct File {
    /// The mime type of the file.
    /// "image/png"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ContentTypeProperty>,
    /// The name of the file. It will be auto-generated if not provided.
    /// "z9RV14K95DvU.png"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<FileNameProperty>,
    /// The size of the file in bytes.
    /// 4404019
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<FileSizeProperty>,
    /// The URL where the file can be downloaded from.
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HTTPValidationError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Vec<Option<ValidationError>>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SoundEffectOutput {
    /// The generated sound effect audio file in MP3 format
    /// {"url":"https://v3.fal.media/files/lion/WgnO-jy6WduosuG_Ibobx_sound_effect.mp3"}
    pub audio: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SoundEffectRequest {
    /// Duration in seconds (0.5-22). If None, optimal duration will be determined from prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<DurationSecondsProperty>,
    /// How closely to follow the prompt (0-1). Higher values mean less variation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_influence: Option<f64>,
    /// The text describing the sound effect to generate
    /// "Spacious braam suitable for high-impact movie trailer moments"
    /// "A gentle wind chime tinkling in a soft breeze"
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SpeechToTextRequest {
    /// URL of the audio file to transcribe
    /// "https://v3.fal.media/files/zebra/zJL_oRY8h5RWwjoK1w7tx_output.mp3"
    pub audio_url: String,
    /// Whether to annotate who is speaking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diarize: Option<bool>,
    /// Language code of the audio
    /// "eng"
    /// "spa"
    /// "fra"
    /// "deu"
    /// "jpn"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCodeProperty>,
    /// Tag audio events like laughter, applause, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_audio_events: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TTSOutput {
    /// The generated audio file
    /// {"url":"https://v3.fal.media/files/zebra/zJL_oRY8h5RWwjoK1w7tx_output.mp3"}
    pub audio: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToSpeechRequest {
    /// Similarity boost (0-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity_boost: Option<f64>,
    /// Voice stability (0-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability: Option<f64>,
    /// Style exaggeration (0-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<f64>,
    /// The text to convert to speech
    /// "Hello! This is a test of the text to speech system, powered by ElevenLabs. How does it sound?"
    pub text: String,
    /// The voice to use for speech generation
    /// "Aria"
    /// "Roger"
    /// "Sarah"
    /// "Laura"
    /// "Charlie"
    /// "George"
    /// "Callum"
    /// "River"
    /// "Liam"
    /// "Charlotte"
    /// "Alice"
    /// "Matilda"
    /// "Will"
    /// "Jessica"
    /// "Eric"
    /// "Chris"
    /// "Brian"
    /// "Daniel"
    /// "Lily"
    /// "Bill"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TranscriptionOutput {
    /// Detected or specified language code
    pub language_code: String,
    /// Confidence in language detection
    pub language_probability: f64,
    /// The full transcribed text
    pub text: String,
    /// Word-level transcription details
    pub words: Vec<TranscriptionWord>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TranscriptionWord {
    /// End time in seconds
    pub end: f64,
    /// Speaker identifier if diarization was enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_id: Option<SpeakerIdProperty>,
    /// Start time in seconds
    pub start: f64,
    /// The transcribed word or audio event
    pub text: String,
    /// Type of element (word, spacing, or audio_event)
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ValidationError {
    pub loc: Vec<serde_json::Value>,
    pub msg: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum LanguageCodeProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum DurationSecondsProperty {
    #[default]
    Number(f64),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum FileSizeProperty {
    #[default]
    Integer(i64),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum FileNameProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum SpeakerIdProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum ContentTypeProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

/// ElevenLabs Audio Isolation
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
    FalRequest::new("fal-ai/elevenlabs/tts/turbo-v2.5", params)
}
