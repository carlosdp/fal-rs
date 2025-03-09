#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct BGRemoveOutput {
    /// The generated image/// The generated image/// {"content_type":"image/png","file_name":"070c731993e949d993c10ef6283d335d.png","file_size":1076276,"height":1024,"url":"https://v3.fal.media/files/tiger/GQEMNjRyxSoza7N8LPPqb_070c731993e949d993c10ef6283d335d.png","width":1024}
    pub image: Image,
}

/// Bria Text-to-Image Base
///
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
pub fn remove(params: BGRemoveInput) -> FalRequest<BGRemoveInput, BGRemoveOutput> {
    FalRequest::new("fal-ai/bria/text-to-image/base", params)
}
