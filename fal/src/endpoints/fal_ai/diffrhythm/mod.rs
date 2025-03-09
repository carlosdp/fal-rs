#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Generated music file./// Generated music file./// {"content_type":"application/octet-stream","file_name":"output.wav","file_size":33554520,"url":"https://v3.fal.media/files/elephant/VV4wtKXBpZL1bNv6en36t_output.wav"}

pub audio: File
    }
    

                /// DiffRhythm: Lyrics to Song
/// 
/// DiffRhythm is a blazing fast model for transforming lyrics into full songs. It boasts the capability to generate full songs in less than 30 seconds.
/// 
/// Category: text-to-audio
/// 
/// License Type: commercial
                pub fn diffrhythm(params: TextToMusicInput) -> FalRequest<TextToMusicInput, Output> {
                    FalRequest::new(
                        "fal-ai/diffrhythm",
                        params
                    )
                }
                