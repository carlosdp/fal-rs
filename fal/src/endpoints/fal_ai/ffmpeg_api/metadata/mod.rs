#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MetadataOutput {
        /// Metadata for the analyzed media file (either Video or Audio)
pub media: MediaProperty
    }
    

                /// FFmpeg API Compose
/// 
/// Compose videos from multiple media sources using FFmpeg API.
/// 
/// Category: video-to-video
/// Machine Type: L
                pub fn metadata(params: MetadataInput) -> FalRequest<MetadataInput, MetadataOutput> {
                    FalRequest::new(
                        "fal-ai/ffmpeg-api/compose",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MetadataOutput {
        /// Metadata for the analyzed media file (either Video or Audio)
pub media: MediaProperty
    }
    

                /// FFmpeg API Metadata
/// 
/// Get encoding metadata from video and audio files using FFmpeg API.
/// 
/// Category: json
/// Machine Type: L
                pub fn metadata(params: MetadataInput) -> FalRequest<MetadataInput, MetadataOutput> {
                    FalRequest::new(
                        "fal-ai/ffmpeg-api/metadata",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MetadataOutput {
        /// Metadata for the analyzed media file (either Video or Audio)
pub media: MediaProperty
    }
    

                /// FFmpeg API Waveform
/// 
/// Get waveform data from audio files using FFmpeg API.
/// 
/// Category: json
/// Machine Type: L
                pub fn metadata(params: MetadataInput) -> FalRequest<MetadataInput, MetadataOutput> {
                    FalRequest::new(
                        "fal-ai/ffmpeg-api/waveform",
                        params
                    )
                }
                