#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BGRemoveInput {
    /// Input Image to erase from
    /// "https://fal.media/files/panda/K5Rndvzmn1j-OI1VZXDVd.jpeg"
    pub image_url: String,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BGRemoveOutput {
    /// The generated image
    /// {"content_type":"image/png","file_name":"070c731993e949d993c10ef6283d335d.png","file_size":1076276,"height":1024,"url":"https://v3.fal.media/files/tiger/GQEMNjRyxSoza7N8LPPqb_070c731993e949d993c10ef6283d335d.png","width":1024}
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BGReplaceInput {
    /// Whether to use the fast model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fast: Option<bool>,
    /// Input Image to erase from
    /// "https://storage.googleapis.com/falserverless/bria/bria_bg_replace_fg.jpg"
    pub image_url: String,
    /// The negative prompt you would like to use to generate images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Number of Images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The prompt you would like to use to generate images.
    /// "Lilypad on a river"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// The URL of the reference image to be used for generating the new background. Use "" to leave empty. Either ref_image_url or bg_prompt has to be provided but not both. If both ref_image_url and ref_image_file are provided, ref_image_url will be used. Accepted formats are jpeg, jpg, png, webp.
    /// "https://storage.googleapis.com/falserverless/bria/bria_bg_replace_bg.jpg"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_image_url: Option<String>,
    /// Whether to refine prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refine_prompt: Option<bool>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BGReplaceOutput {
    /// The generated images
    /// [{"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_bg_replace_res.jpg"}]
    pub images: Vec<Image>,
    /// Seed value used for generation.
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EraserInput {
    /// Input Image to erase from
    /// "https://storage.googleapis.com/falserverless/bria/bria_eraser_img.png"
    pub image_url: String,
    /// You can use this parameter to specify the type of the input mask from the list. 'manual' opttion should be used in cases in which the mask had been generated by a user (e.g. with a brush tool), and 'automatic' mask type should be used when mask had been generated by an algorithm like 'SAM'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_type: Option<String>,
    /// The URL of the binary mask image that represents the area that will be cleaned.
    /// "https://storage.googleapis.com/falserverless/bria/bria_eraser_mask.png"
    pub mask_url: String,
    /// If set to true, attempts to preserve the alpha channel of the input image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_alpha: Option<bool>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EraserOutput {
    /// The generated image
    /// {"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_eraser_res.png"}
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FastTextToImageRequest {
    /// The aspect ratio of the image. When a guidance method is being used, the aspect ratio is defined by the guidance image and this parameter is ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// Guidance images to use for the generation. Up to 4 guidance methods can be combined during a single inference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance: Option<Vec<Option<GuidanceInput>>>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// Which medium should be included in your generated images. This parameter is optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    /// The negative prompt you would like to use to generate images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// How many images you would like to generate. When using any Guidance Method, Value is set to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of iterations the model goes through to refine the generated image. This parameter is optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt you would like to use to generate images.
    /// "A lone figure stands on the edge of a serene cliff at sunset, gazing out over a vast, mystical valley. The figure is clad in flowing robes that ripple in the gentle breeze, silhouetted against the golden and lavender hues of the sky. Below, a cascading waterfall pours into a sparkling river winding through a forest of bioluminescent trees. The scene blends the awe of nature with a touch of otherworldly wonder, inviting reflection and imagination."
    pub prompt: String,
    /// When set to true, enhances the provided prompt by generating additional, more descriptive variations, resulting in more diverse and creative output images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_enhancement: Option<bool>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GenFillInput {
    /// Input Image to erase from
    /// "https://storage.googleapis.com/falserverless/bria/bria_genfill_img.png"
    pub image_url: String,
    /// The URL of the binary mask image that represents the area that will be cleaned.
    /// "https://storage.googleapis.com/falserverless/bria/bria_genfill_mask.png"
    pub mask_url: String,
    /// The negative prompt you would like to use to generate images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Number of Images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The prompt you would like to use to generate images.
    /// "A red delicious cherry"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GenFillOutput {
    /// Generated Images
    /// [{"content_type":"image/png","file_name":"a0d138e6820c4ad58f1fd3c758f16047.png","file_size":1064550,"height":768,"url":"https://storage.googleapis.com/falserverless/bria/bria_genfill_res.png","width":1024}]
    pub images: Vec<Image>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GuidanceInput {
    /// The image that should be used as guidance, in base64 format, with the method defined in guidance_method_1. Accepted formats are jpeg, jpg, png, webp. Maximum file size 12MB. If more then one guidance method is used, all guidance images must be of the same aspect ratio, and this will be the aspect ratio of the generated results. If guidance_method_1 is selected, an image must be provided.
    pub image_url: String,
    /// Which guidance type you would like to include in the generation. Up to 4 guidance methods can be combined during a single inference. This parameter is optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Impact of the guidance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HTTPValidationError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Vec<Option<ValidationError>>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Image {
    /// The mime type of the file.
    /// "image/png"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// File data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>,
    /// The name of the file. It will be auto-generated if not provided.
    /// "z9RV14K95DvU.png"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// The size of the file in bytes.
    /// 4404019
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// The height of the image in pixels.
    /// 1024
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// The URL where the file can be downloaded from.
    pub url: String,
    /// The width of the image in pixels.
    /// 1024
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageExpansionInput {
    /// The desired size of the final image, after the expansion. should have an area of less than 5000x5000 pixels.
    /// [1200,674]
    pub canvas_size: Vec<i64>,
    /// The URL of the input image.
    /// "https://storage.googleapis.com/falserverless/model_tests/orange.png"
    pub image_url: String,
    /// The negative prompt you would like to use to generate images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Number of Images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The desired location of the original image, inside the full canvas. Provide the location of the upper left corner of the original image. The location can also be outside the canvas (the original image will be cropped).
    /// [301,-66]
    pub original_image_location: Vec<i64>,
    /// The desired size of the original image, inside the full canvas. Ensure that the ratio of input image foreground or main subject to the canvas area is greater than 15% to achieve optimal results.
    /// [610,855]
    pub original_image_size: Vec<i64>,
    /// Text on which you wish to base the image expansion. This parameter is optional. Bria currently supports prompts in English only, excluding special characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// You can choose whether you want your generated expension to be random or predictable. You can recreate the same result in the future by using the seed value of a result from the response. You can exclude this parameter if you are not interested in recreating your results. This parameter is optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageExpansionOutput {
    /// The generated image
    /// {"content_type":"image/png","file_name":"afa402a35ea742cdb5c3e219b2b19bfb.png","file_size":1471342,"height":674,"url":"https://v3.fal.media/files/koala/8np-spgxxG-I1r3cjthRV_afa402a35ea742cdb5c3e219b2b19bfb.png","width":1200}
    pub image: Image,
    /// Seed value used for generation.
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Output {
    /// The generated images
    /// [{"content_type":"image/png","file_name":"257cf8e7bd3a47c2959396343d5b38cf.png","file_size":3731290,"height":1536,"url":"https://v3.fal.media/files/tiger/48e63e0K6C9XQYBuomoU-_257cf8e7bd3a47c2959396343d5b38cf.png","width":1536}]
    pub images: Vec<Image>,
    /// Seed value used for generation.
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProductShotInput {
    /// Whether to use the fast model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fast: Option<bool>,
    /// The URL of the product shot to be placed in a lifestyle shot. If both image_url and image_file are provided, image_url will be used. Accepted formats are jpeg, jpg, png, webp. Maximum file size 12MB.
    /// "https://storage.googleapis.com/falserverless/bria/bria_product_fg.jpg"
    pub image_url: String,
    /// If you've selected placement_type=manual_placement, you should use this parameter to specify which placements/positions you would like to use from the list. You can select more than one placement in one request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_placement_selection: Option<String>,
    /// The number of lifestyle product shots you would like to generate. You will get num_results x 10 results when placement_type=automatic and according to the number of required placements x num_results if placement_type=manual_placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
    /// Whether to optimize the scene description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimize_description: Option<bool>,
    /// This flag is only relevant when placement_type=original. If true, the output image retains the original input image's size; otherwise, the image is scaled to 1 megapixel (1MP) while preserving its aspect ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_quality: Option<bool>,
    /// The desired padding in pixels around the product, when using placement_type=manual_padding. The order of the values is [left, right, top, bottom]. For optimal results, the total number of pixels, including padding, should be around 1,000,000. It is recommended to first use the product cutout API, get the cutout and understand the size of the result, and then define the required padding and use the cutout as an input for this API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_values: Option<Vec<Option<i64>>>,
    /// This parameter allows you to control the positioning of the product in the image. Choosing 'original' will preserve the original position of the product in the image. Choosing 'automatic' will generate results with the 10 recommended positions for the product. Choosing 'manual_placement' will allow you to select predefined positions (using the parameter 'manual_placement_selection'). Selecting 'manual_padding' will allow you to control the position and size of the image by defining the desired padding in pixels around the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_type: Option<String>,
    /// The URL of the reference image to be used for generating the new scene or background for the product shot. Use "" to leave empty.Either ref_image_url or scene_description has to be provided but not both. If both ref_image_url and ref_image_file are provided, ref_image_url will be used. Accepted formats are jpeg, jpg, png, webp.
    /// "https://storage.googleapis.com/falserverless/bria/bria_product_bg.jpg"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_image_url: Option<String>,
    /// Text description of the new scene or background for the provided product shot. Bria currently supports prompts in English only, excluding special characters.
    /// "on a rock, next to the ocean, dark theme"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_description: Option<String>,
    /// The desired size of the final product shot. For optimal results, the total number of pixels should be around 1,000,000. This parameter is only relevant when placement_type=automatic or placement_type=manual_placement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shot_size: Option<Vec<Option<i64>>>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProductShotOutput {
    /// The generated images
    /// [{"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_product_res.png"}]
    pub images: Vec<Image>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReimagineInput {
    /// Whether to use the fast model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fast: Option<bool>,
    /// The number of iterations the model goes through to refine the generated image. This parameter is optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// How many images you would like to generate. When using any Guidance Method, Value is set to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i64>,
    /// The prompt you would like to use to generate images.
    /// "A 2d illustration of a dog in a vibrant park"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The URL of the structure reference image. Use "" to leave empty. Accepted formats are jpeg, jpg, png, webp.
    /// "https://storage.googleapis.com/falserverless/bria/bria_reimagine_input.png"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure_image_url: Option<String>,
    /// The influence of the structure reference on the generated image.
    /// 0.15
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure_ref_influence: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReimagineOutput {
    /// The generated images
    /// [{"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_reimagine_output.png"}]
    pub images: Vec<Image>,
    /// Seed value used for generation.
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToImageRequest {
    /// The aspect ratio of the image. When a guidance method is being used, the aspect ratio is defined by the guidance image and this parameter is ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// Guidance images to use for the generation. Up to 4 guidance methods can be combined during a single inference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance: Option<Vec<Option<GuidanceInput>>>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// Which medium should be included in your generated images. This parameter is optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    /// The negative prompt you would like to use to generate images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// How many images you would like to generate. When using any Guidance Method, Value is set to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of iterations the model goes through to refine the generated image. This parameter is optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt you would like to use to generate images.
    /// "A lone figure stands on the edge of a serene cliff at sunset, gazing out over a vast, mystical valley. The figure is clad in flowing robes that ripple in the gentle breeze, silhouetted against the golden and lavender hues of the sky. Below, a cascading waterfall pours into a sparkling river winding through a forest of bioluminescent trees. The scene blends the awe of nature with a touch of otherworldly wonder, inviting reflection and imagination."
    pub prompt: String,
    /// When set to true, enhances the provided prompt by generating additional, more descriptive variations, resulting in more diverse and creative output images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_enhancement: Option<bool>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpscaleInput {
    /// The desired increase in resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_increase: Option<String>,
    /// Input Image to erase from
    /// "https://storage.googleapis.com/falserverless/model_tests/remove_background/elephant.jpg"
    pub image_url: String,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpscaleOutput {
    /// Result Image
    /// {"content_type":"image/png","file_name":"db12c5f7076844d0bb84df92ab340acd.png","file_size":2494064,"height":1400,"url":"https://v3.fal.media/files/penguin/oHW1CIjw26zf3Jt-YLBTW_db12c5f7076844d0bb84df92ab340acd.png","width":1220}
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ValidationError {
    pub loc: Vec<serde_json::Value>,
    pub msg: String,
    #[serde(rename = "type")]
    pub ty: String,
}

/// Bria Text-to-Image Base
///
/// Category: text-to-image
/// Machine Type: H100
/// License Type: commercial
pub fn replace(params: BGReplaceInput) -> FalRequest<BGReplaceInput, BGReplaceOutput> {
    FalRequest::new("fal-ai/bria/background/replace", params)
}
