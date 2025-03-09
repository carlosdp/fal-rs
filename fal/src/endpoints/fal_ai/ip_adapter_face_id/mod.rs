#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct IpAdapterFaceIdOutput {
    /// The generated image file info.
    pub image: Image,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
}

/// IP Adapter Face ID
///
/// Category: image-to-image
/// Machine Type: A6000
/// License Type: research
pub fn ip_adapter_face_id(
    params: IpAdapterFaceIdInput,
) -> FalRequest<IpAdapterFaceIdInput, IpAdapterFaceIdOutput> {
    FalRequest::new("fal-ai/ip-adapter-face-id", params)
}
