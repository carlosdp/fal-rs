#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OutputModel {
        /// The generated video with the lip sync./// The generated video with the lip sync./// {"content_type":"video/mp4","file_name":"output.mp4","file_size":120000,"url":"https://v3.fal.media/files/koala/7BzEwUucbr6yuFjpcJipl_output.mp4"}

pub video: File
    }
    

                /// Dubbing
/// 
/// This endpoint delivers seamlessly localized videos by generating lip-synced dubs in multiple languages, ensuring natural and immersive multilingual experiences
/// 
/// Category: video-to-video
/// 
/// License Type: commercial
                pub fn dubbing(params: InputModel) -> FalRequest<InputModel, OutputModel> {
                    FalRequest::new(
                        "fal-ai/dubbing",
                        params
                    )
                }
                