#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ZonosOutput {
        /// The generated audio
pub audio: File
    }
    

                /// Zonos-Audio-Clone
/// 
/// Clone voice of any person and speak anything in their voise using zonos' voice cloning.
/// 
/// Category: text-to-audio
/// Machine Type: A100
                pub fn zonos(params: ZonosInput) -> FalRequest<ZonosInput, ZonosOutput> {
                    FalRequest::new(
                        "fal-ai/zonos",
                        params
                    )
                }
                