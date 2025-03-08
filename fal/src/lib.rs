pub mod request;

pub use fal_derive::endpoint;

use std::{
    future::Future,
    io::{Cursor, Read},
};

use base64::prelude::*;
use futures::{stream::FuturesUnordered, TryFutureExt, TryStreamExt};
use image::{DynamicImage, ImageFormat};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum FalError {
    #[error("fal request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("image error: {0}")]
    ImageError(#[from] image::ImageError),
    #[error("error: {0}")]
    Other(String),
}

impl From<String> for FalError {
    fn from(s: String) -> Self {
        FalError::Other(s)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FalImage {
    pub url: String,
    pub content_type: String,
    pub file_name: Option<String>,
    pub file_size: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FalSingleImageResponse {
    pub image: FalImage,
}

impl IntoImages for FalSingleImageResponse {
    fn into_images(self) -> impl Future<Output = Result<Vec<image::DynamicImage>, FalError>> {
        async move {
            let image_bytes = reqwest::get(&self.image.url).await?.bytes().await?;
            let output = image::load_from_memory(&image_bytes)?;
            Ok(vec![output])
        }
    }

    fn into_raw_images(self) -> impl Future<Output = Result<Vec<Vec<u8>>, FalError>> {
        async move {
            Ok(vec![reqwest::get(&self.image.url)
                .await?
                .bytes()
                .await?
                .to_vec()])
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FalMultiImageResponse {
    pub images: Vec<FalImage>,
}

impl IntoImages for FalMultiImageResponse {
    fn into_images(self) -> impl Future<Output = Result<Vec<image::DynamicImage>, FalError>> {
        async move {
            let mut result = Vec::new();

            for image in &self.images {
                let image_bytes = reqwest::get(&image.url).await?.bytes().await?;
                let output = image::load_from_memory(&image_bytes)?;
                result.push(output);
            }

            Ok(result)
        }
    }

    fn into_raw_images(self) -> impl Future<Output = Result<Vec<Vec<u8>>, FalError>> {
        async move {
            Ok(FuturesUnordered::from_iter(
                self.images
                    .iter()
                    .map(|image| reqwest::get(&image.url).and_then(|r| r.bytes())),
            )
            .try_collect::<Vec<_>>()
            .await?
            .into_iter()
            .map(|b| b.to_vec())
            .collect())
        }
    }
}

pub trait IntoImages {
    fn into_images(self) -> impl Future<Output = Result<Vec<image::DynamicImage>, FalError>>;

    fn into_raw_images(self) -> impl Future<Output = Result<Vec<Vec<u8>>, FalError>>;
}

pub async fn fal_generate<R: DeserializeOwned + IntoImages>(
    endpoint: &str,
    params: serde_json::Value,
) -> Result<Vec<image::DynamicImage>, FalError> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("https://fal.run/{}", endpoint))
        .json(&params)
        .header(
            "Authorization",
            format!("Key {}", std::env::var("FAL_API_KEY").unwrap()),
        )
        .header("Content-Type", "application/json")
        .send()
        .await?;

    if response.status() != 200 {
        let error = response.text().await?;
        return Err(error.into());
    }

    let response = response.error_for_status()?.json::<R>().await?;

    Ok(response.into_images().await?)
}

pub async fn fal_generate_raw<R: DeserializeOwned + IntoImages>(
    endpoint: &str,
    params: serde_json::Value,
) -> Result<Vec<Vec<u8>>, FalError> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("https://fal.run/{}", endpoint))
        .json(&params)
        .header(
            "Authorization",
            format!("Key {}", std::env::var("FAL_API_KEY").unwrap()),
        )
        .header("Content-Type", "application/json")
        .send()
        .await?;

    if response.status() != 200 {
        let error = response.text().await?;
        return Err(error.into());
    }

    let response = response.error_for_status()?.json::<R>().await?;

    Ok(response.into_raw_images().await?)
}

pub async fn fal_bg_remove(image: image::DynamicImage) -> Result<image::DynamicImage, FalError> {
    let data_url = image_to_data_url(&image);
    let params = serde_json::json!({
        "image_url": data_url,
    });

    let client = reqwest::Client::new();
    let response = client
        .post("https://fal.run/fal-ai/imageutils/rembg")
        .json(&params)
        .header(
            "Authorization",
            format!("Key {}", std::env::var("FAL_API_KEY").unwrap()),
        )
        .header("Content-Type", "application/json")
        .send()
        .await?;

    if response.status() != 200 {
        let error = response.text().await?;
        return Err(error.into());
    }

    let response = response
        .error_for_status()?
        .json::<FalSingleImageResponse>()
        .await?;

    let image_bytes = reqwest::get(&response.image.url).await?.bytes().await?;
    let image = image::load_from_memory(&image_bytes)?;

    Ok(image)
}

pub trait ToDataUrl {
    fn to_data_url(&self) -> String;
}

pub trait ToRead {
    fn to_read(&self) -> impl Read;
}

impl ToDataUrl for DynamicImage {
    fn to_data_url(&self) -> String {
        image_to_data_url(self)
    }
}

impl ToRead for DynamicImage {
    fn to_read(&self) -> impl Read {
        image_to_read(self)
    }
}

pub fn image_to_data_url(image: &image::DynamicImage) -> String {
    let mut buf = Vec::new();
    let mut writer = Cursor::new(&mut buf);
    image
        .write_to(&mut writer, image::ImageFormat::Png)
        .unwrap();
    let data = BASE64_STANDARD.encode(&buf);
    format!("data:image/jpeg;base64,{}", data)
}

fn image_to_read(image: &DynamicImage) -> impl Read {
    let mut buf = Vec::new();
    let mut writer = Cursor::new(&mut buf);
    image.write_to(&mut writer, ImageFormat::Png).unwrap();

    Cursor::new(buf)
}

#[endpoint(endpoint = "fal-ai/imageutils/rembg", in_fal_crate)]
pub fn rembg(image_url: String) -> FalSingleImageResponse {}
