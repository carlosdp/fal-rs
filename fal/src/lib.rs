#[cfg(feature = "endpoints")]
pub mod endpoints;
#[cfg(feature = "image")]
pub mod image;
pub mod request;

pub use fal_derive::endpoint;

pub mod prelude {
    #[cfg(feature = "endpoints")]
    pub use super::endpoints::*;
    #[cfg(feature = "image")]
    pub use super::image::*;
    pub use super::request::*;
    pub use super::*;
}

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

pub type Image = File;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct File {
    pub url: String,
    pub content_type: String,
    pub file_name: Option<String>,
    pub file_size: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FalSingleImageResponse {
    pub image: File,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FalMultiImageResponse {
    pub images: Vec<File>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Timings {
    // todo
}
