#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// Generated music file.
    /// {"content_type":"application/octet-stream","file_name":"output.wav","file_size":33554520,"url":"https://v3.fal.media/files/elephant/VV4wtKXBpZL1bNv6en36t_output.wav"}
    pub audio: File,
}

/// DiffRhythm: Lyrics to Song
///
/// Category: text-to-audio
///
/// License Type: commercial
pub fn diffrhythm(params: TextToMusicInput) -> FalRequest<TextToMusicInput, Output> {
    FalRequest::new("fal-ai/diffrhythm", params)
}
