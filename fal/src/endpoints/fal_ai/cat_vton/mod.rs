#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CATVTONOutput {
    /// The output image.
    pub image: Image,
}

/// try-on
///
/// Category: image-to-image
/// Machine Type: A100
/// License Type: research
pub fn cat_vton(params: CATVTONInput) -> FalRequest<CATVTONInput, CATVTONOutput> {
    FalRequest::new("fal-ai/cat-vton", params)
}
