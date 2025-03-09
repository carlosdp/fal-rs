use fal::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FalMultiImageResponse {
    pub images: Vec<File>,
}

#[fal::endpoint(endpoint = "fal-ai/flux/dev")]
fn fal_dev(prompt: String) -> FalMultiImageResponse {}

#[tokio::test]
async fn test_into_image() {
    let mut response = fal_dev("a horse".to_owned()).send().await.unwrap();

    assert!(response.images.len() > 0);

    response.images.remove(0).into_image().await.unwrap();
}

#[tokio::test]
async fn test_into_raw_image() {
    let mut response = fal_dev("a horse".to_owned()).send().await.unwrap();

    assert!(response.images.len() > 0);

    response.images.remove(0).into_raw_image().await.unwrap();
}
