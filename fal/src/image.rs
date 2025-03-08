use std::{
    future::Future,
    io::{Cursor, Read},
};

use base64::prelude::*;
use image::{DynamicImage, ImageFormat};

use crate::{FalError, File};

impl File {
    pub fn into_image(self) -> impl Future<Output = Result<image::DynamicImage, FalError>> {
        async move {
            let image_bytes = reqwest::get(&self.url).await?.bytes().await?;
            let output = image::load_from_memory(&image_bytes)?;
            Ok(output)
        }
    }

    pub fn into_raw_image(self) -> impl Future<Output = Result<Vec<u8>, FalError>> {
        async move {
            let image_bytes = reqwest::get(&self.url).await?.bytes().await?;
            Ok(image_bytes.to_vec())
        }
    }
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
