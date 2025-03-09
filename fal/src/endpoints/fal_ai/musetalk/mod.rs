#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MuseTalkOutput {
        /// The generated video file.
pub video: File
    }
    

                /// MuseTalk
/// 
/// MuseTalk is a real-time high quality audio-driven lip-syncing model. Use MuseTalk to animate a face with your own audio.
/// 
/// Category: image-to-video
/// Machine Type: A100
                pub fn musetalk(params: MuseTalkInput) -> FalRequest<MuseTalkInput, MuseTalkOutput> {
                    FalRequest::new(
                        "fal-ai/musetalk",
                        params
                    )
                }
                