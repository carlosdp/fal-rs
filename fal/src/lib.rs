#[cfg(feature = "image")]
pub mod image;
pub mod request;

pub use fal_derive::endpoint;

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum FalError {
    #[error("fal request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("image error: {0}")]
    ImageError(#[from] ::image::ImageError),
    #[error("error: {0}")]
    Other(String),
}

impl From<String> for FalError {
    fn from(s: String) -> Self {
        FalError::Other(s)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FalFile {
    pub url: String,
    pub content_type: String,
    pub file_name: Option<String>,
    pub file_size: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FalSingleImageResponse {
    pub image: FalFile,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FalMultiImageResponse {
    pub images: Vec<FalFile>,
}
