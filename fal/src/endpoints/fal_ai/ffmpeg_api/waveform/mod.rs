#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct WaveformOutput {
        /// Duration of the audio in seconds
pub duration: f64,
/// Number of points in the waveform data
pub points: i64,
/// Number of decimal places used in the waveform values
pub precision: i64,
/// Normalized waveform data as an array of values between -1 and 1. The number of points is determined by audio duration × points_per_second.
pub waveform: Vec<f64>
    }
    

                /// FFmpeg API Compose
/// 
/// Compose videos from multiple media sources using FFmpeg API.
/// 
/// Category: video-to-video
/// Machine Type: L
                pub fn waveform(params: WaveformInput) -> FalRequest<WaveformInput, WaveformOutput> {
                    FalRequest::new(
                        "fal-ai/ffmpeg-api/compose",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct WaveformOutput {
        /// Duration of the audio in seconds
pub duration: f64,
/// Number of points in the waveform data
pub points: i64,
/// Number of decimal places used in the waveform values
pub precision: i64,
/// Normalized waveform data as an array of values between -1 and 1. The number of points is determined by audio duration × points_per_second.
pub waveform: Vec<f64>
    }
    

                /// FFmpeg API Metadata
/// 
/// Get encoding metadata from video and audio files using FFmpeg API.
/// 
/// Category: json
/// Machine Type: L
                pub fn waveform(params: WaveformInput) -> FalRequest<WaveformInput, WaveformOutput> {
                    FalRequest::new(
                        "fal-ai/ffmpeg-api/metadata",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct WaveformOutput {
        /// Duration of the audio in seconds
pub duration: f64,
/// Number of points in the waveform data
pub points: i64,
/// Number of decimal places used in the waveform values
pub precision: i64,
/// Normalized waveform data as an array of values between -1 and 1. The number of points is determined by audio duration × points_per_second.
pub waveform: Vec<f64>
    }
    

                /// FFmpeg API Waveform
/// 
/// Get waveform data from audio files using FFmpeg API.
/// 
/// Category: json
/// Machine Type: L
                pub fn waveform(params: WaveformInput) -> FalRequest<WaveformInput, WaveformOutput> {
                    FalRequest::new(
                        "fal-ai/ffmpeg-api/waveform",
                        params
                    )
                }
                