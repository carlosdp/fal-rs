#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductShotOutput {
    /// The generated images/// The generated images/// [{"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_product_res.png"}]
    pub images: Vec<Image>,
}

/// Bria Text-to-Image Base
///
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
pub fn product_shot(params: ProductShotInput) -> FalRequest<ProductShotInput, ProductShotOutput> {
    FalRequest::new("fal-ai/bria/text-to-image/base", params)
}
