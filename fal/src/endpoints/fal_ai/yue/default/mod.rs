use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// Generated music file./// Generated music file./// {"content_type":"application/octet-stream","file_name":"cot_inspiring-female-uplifting-pop-airy-vocal-electronic-bright-vocal-vocal_tp0@93_T1@0_rp1@2_maxtk3000_mixed_8179e8da-5452-4cf6-9d6b-f69280feb7e8.mp3","file_size":480462,"url":"https://v3.fal.media/files/tiger/iAXHU3LtbJGeqPYWKkYMr_cot_inspiring-female-uplifting-pop-airy-vocal-electronic-bright-vocal-vocal_tp0%4093_T1%400_rp1%402_maxtk3000_mixed_74bcf408-eb99-4b88-b7bf-7d7212200cf1.mp3"}
    pub audio: File,
}

/// YuE: Lyrics to Song
///
/// YuE is a groundbreaking series of open-source foundation models designed for music generation, specifically for transforming lyrics into full songs.
///
/// Category: text-to-audio
///
/// License Type: commercial
pub fn default(params: TextToMusicInput) -> FalRequest<TextToMusicInput, Output> {
    FalRequest::new("fal-ai/yue", params)
}
