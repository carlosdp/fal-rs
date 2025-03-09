#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FaceToStickerOutput {
        /// Whether the generated images contain NSFW concepts.
/// The key is the image type and the value is a boolean./// Whether the generated images contain NSFW concepts.
/// The key is the image type and the value is a boolean./// {"sticker_image":false,"sticker_image_background_removed":false}

pub has_nsfw_concepts: Has Nsfw Concepts,
/// The generated images./// The generated images./// [{"content_type":"image/PNG","file_name":"cd8bab71b946470099d5fa20c7eed440.png","file_size":560358,"height":1024,"url":"https://storage.googleapis.com/falserverless/model_tests/face_to_sticker/elon_output_1.png","width":1024},{"content_type":"image/PNG","file_name":"181ae8fa12534c6f9285a991b415d9a7.png","file_size":452906,"height":1024,"url":"https://storage.googleapis.com/falserverless/model_tests/face_to_sticker/elon_output_2.png","width":1024}]

pub images: Vec<Image>,
/// Seed used during the inference./// Seed used during the inference./// 3625437076

pub seed: i64,
/// The generated face sticker image./// The generated face sticker image./// {"content_type":"image/PNG","file_name":"cd8bab71b946470099d5fa20c7eed440.png","file_size":560358,"height":1024,"url":"https://storage.googleapis.com/falserverless/model_tests/face_to_sticker/elon_output_1.png","width":1024}

pub sticker_image: Image,
/// The generated face sticker image with the background removed./// The generated face sticker image with the background removed./// {"content_type":"image/PNG","file_name":"181ae8fa12534c6f9285a991b415d9a7.png","file_size":452906,"height":1024,"url":"https://storage.googleapis.com/falserverless/model_tests/face_to_sticker/elon_output_2.png","width":1024}

pub sticker_image_background_removed: Image
    }
    

                /// Face to Sticker
/// 
/// Create stickers from faces.
/// 
/// Category: image-to-image
/// Machine Type: A100
/// License Type: research
                pub fn face_to_sticker(params: FaceToStickerInput) -> FalRequest<FaceToStickerInput, FaceToStickerOutput> {
                    FalRequest::new(
                        "fal-ai/face-to-sticker",
                        params
                    )
                }
                