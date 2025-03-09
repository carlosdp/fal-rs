use fal::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FalMultiImageResponse {
    pub images: Vec<File>,
}

#[fal::endpoint(endpoint = "fal-ai/flux/dev")]
fn fal_dev(prompt: String) -> FalMultiImageResponse {}

#[tokio::test]
async fn test_simple_send() {
    let response = fal_dev("a horse".to_owned()).send().await.unwrap();

    assert!(response.images.len() > 0)
}
