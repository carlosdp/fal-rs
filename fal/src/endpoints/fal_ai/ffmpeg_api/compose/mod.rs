#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// URL of the video's thumbnail image
pub thumbnail_url: String,
/// URL of the processed video file
pub video_url: String
    }
    

                /// FFmpeg API Compose
/// 
/// Compose videos from multiple media sources using FFmpeg API.
/// 
/// Category: video-to-video
/// Machine Type: L
                pub fn compose(params: Input) -> FalRequest<Input, Output> {
                    FalRequest::new(
                        "fal-ai/ffmpeg-api/compose",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// URL of the video's thumbnail image
pub thumbnail_url: String,
/// URL of the processed video file
pub video_url: String
    }
    

                /// FFmpeg API Metadata
/// 
/// Get encoding metadata from video and audio files using FFmpeg API.
/// 
/// Category: json
/// Machine Type: L
                pub fn compose(params: Input) -> FalRequest<Input, Output> {
                    FalRequest::new(
                        "fal-ai/ffmpeg-api/metadata",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// URL of the video's thumbnail image
pub thumbnail_url: String,
/// URL of the processed video file
pub video_url: String
    }
    

                /// FFmpeg API Waveform
/// 
/// Get waveform data from audio files using FFmpeg API.
/// 
/// Category: json
/// Machine Type: L
                pub fn compose(params: Input) -> FalRequest<Input, Output> {
                    FalRequest::new(
                        "fal-ai/ffmpeg-api/waveform",
                        params
                    )
                }
                