use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProPlusTextToImageInput {
    /// If set to true, the safety checker will be enabled.
    pub enable_safety_checker: Option<bool>,
    /// The size of the generated image.
    pub image_size: Option<ImageSizeProperty>,
    /// The number of images to generate.
    pub num_images: Option<i64>,
    /// The format of the generated image.
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    pub safety_tolerance: Option<String>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LipSyncOutput {
    /// The generated video/// The generated video/// {"url":"https://v3.fal.media/files/rabbit/6gJV-z7RJsF0AxkZHkdgJ_output.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProUltraTextToImageInputRedux {
    /// The aspect ratio of the generated image.
    pub aspect_ratio: Option<AspectRatioProperty>,
    /// If set to true, the safety checker will be enabled.
    pub enable_safety_checker: Option<bool>,
    /// The strength of the image prompt, between 0 and 1.
    pub image_prompt_strength: Option<f64>,
    /// The image URL to generate an image from. Needs to match the dimensions of the mask./// The image URL to generate an image from. Needs to match the dimensions of the mask./// "https://fal.media/files/kangaroo/acQvq-Kmo2lajkgvcEHdv.png"
    pub image_url: String,
    /// The number of images to generate.
    pub num_images: Option<i64>,
    /// The format of the generated image.
    pub output_format: Option<String>,
    /// The prompt to generate an image from.
    pub prompt: Option<String>,
    /// Generate less processed, more natural-looking images.
    pub raw: Option<bool>,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    pub safety_tolerance: Option<String>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MusicOutput {
    /// The generated music/// The generated music/// {"url":"https://fal.media/files/elephant/N5UNLCwkC2y8v7a3LQLFE_output.mp3"}
    pub audio: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProUltraTextToImageFinetunedInput {
    /// The aspect ratio of the generated image.
    pub aspect_ratio: Option<AspectRatioProperty>,
    /// If set to true, the safety checker will be enabled.
    pub enable_safety_checker: Option<bool>,
    /// References your specific model
    pub finetune_id: String,
    /// Controls finetune influence.
    /// Increase this value if your target concept isn't showing up strongly enough.
    /// The optimal setting depends on your finetune and prompt
    pub finetune_strength: f64,
    /// The number of images to generate.
    pub num_images: Option<i64>,
    /// The format of the generated image.
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
    pub prompt: String,
    /// Generate less processed, more natural-looking images.
    pub raw: Option<bool>,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    pub safety_tolerance: Option<String>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProDepthControlFinetunedInput {
    /// The control image URL to generate the depth map from./// The control image URL to generate the depth map from./// "https://fal.media/files/penguin/vt-SeIOweN7_oYBsvGO6t.png"
    pub control_image_url: String,
    /// References your specific model
    pub finetune_id: String,
    /// Controls finetune influence.
    /// Increase this value if your target concept isn't showing up strongly enough.
    /// The optimal setting depends on your finetune and prompt
    pub finetune_strength: f64,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    pub image_size: Option<ImageSizeProperty>,
    /// The number of images to generate.
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "A blackhole in space."
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    pub safety_tolerance: Option<String>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProFillInput {
    /// The image URL to generate an image from. Needs to match the dimensions of the mask./// The image URL to generate an image from. Needs to match the dimensions of the mask./// "https://storage.googleapis.com/falserverless/flux-lora/example-images/knight.jpeg"
    pub image_url: String,
    /// The mask URL to inpaint the image. Needs to match the dimensions of the input image./// The mask URL to inpaint the image. Needs to match the dimensions of the input image./// "https://storage.googleapis.com/falserverless/flux-lora/example-images/mask_knight.jpeg"
    pub mask_url: String,
    /// The number of images to generate.
    pub num_images: Option<i64>,
    /// The format of the generated image.
    pub output_format: Option<String>,
    /// The prompt to fill the masked part of the image./// The prompt to fill the masked part of the image./// "A knight in shining armour holding a greatshield with \"FAL\" on it"
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    pub safety_tolerance: Option<String>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageSize {
    /// The height of the generated image.
    pub height: Option<i64>,
    /// The width of the generated image.
    pub width: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ValidationError {
    pub loc: Vec<serde_json::Value>,
    pub msg: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProFillFinetunedInput {
    /// References your specific model
    pub finetune_id: String,
    /// Controls finetune influence.
    /// Increase this value if your target concept isn't showing up strongly enough.
    /// The optimal setting depends on your finetune and prompt
    pub finetune_strength: f64,
    /// The image URL to generate an image from. Needs to match the dimensions of the mask./// The image URL to generate an image from. Needs to match the dimensions of the mask./// "https://storage.googleapis.com/falserverless/flux-lora/example-images/knight.jpeg"
    pub image_url: String,
    /// The mask URL to inpaint the image. Needs to match the dimensions of the input image./// The mask URL to inpaint the image. Needs to match the dimensions of the input image./// "https://storage.googleapis.com/falserverless/flux-lora/example-images/mask_knight.jpeg"
    pub mask_url: String,
    /// The number of images to generate.
    pub num_images: Option<i64>,
    /// The format of the generated image.
    pub output_format: Option<String>,
    /// The prompt to fill the masked part of the image./// The prompt to fill the masked part of the image./// "A knight in shining armour holding a greatshield with \"FAL\" on it"
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    pub safety_tolerance: Option<String>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProRedux {
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    pub image_size: Option<ImageSizeProperty>,
    /// The image URL to generate an image from. Needs to match the dimensions of the mask./// The image URL to generate an image from. Needs to match the dimensions of the mask./// "https://fal.media/files/kangaroo/acQvq-Kmo2lajkgvcEHdv.png"
    pub image_url: String,
    /// The number of images to generate.
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    pub output_format: Option<String>,
    /// The prompt to generate an image from.
    pub prompt: Option<String>,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    pub safety_tolerance: Option<String>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LipSyncInput {
    /// URL of the input audio/// URL of the input audio/// "https://fal.media/files/lion/vyFWygmZsIZlUO4s0nr2n.wav"
    pub audio_url: String,
    /// The model to use for lipsyncing
    pub model: Option<String>,
    /// Lipsync mode when audio and video durations are out of sync.
    pub sync_mode: Option<String>,
    /// URL of the input video/// URL of the input video/// "https://fal.media/files/koala/8teUPbRRMtAUTORDvqy0l.mp4"
    pub video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToMusicRequest {
    /// Lyrics with optional formatting. You can use a newline to separate each line of lyrics. You can use two newlines to add a pause between lines. You can use double hash marks (##) at the beginning and end of the lyrics to add accompaniment. Maximum 600 characters./// Lyrics with optional formatting. You can use a newline to separate each line of lyrics. You can use two newlines to add a pause between lines. You can use double hash marks (##) at the beginning and end of the lyrics to add accompaniment. Maximum 600 characters./// "## Fast and Limitless   \n In the heart of the code, where dreams collide,   \n\nFAL's the name, taking tech for a ride.    \nGenerative media, blazing the trail,   \n\nFast inference power, we'll never fail.\n##"
    pub prompt: String,
    /// Reference song, should contain music and vocals. Must be a .wav or .mp3 file longer than 15 seconds./// Reference song, should contain music and vocals. Must be a .wav or .mp3 file longer than 15 seconds./// "https://fal.media/files/lion/OOTBTSlxKMH_E8H6hoSlb.mpga"
    pub reference_audio_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProCannyControlInput {
    /// The control image URL to generate the Canny edge map from./// The control image URL to generate the Canny edge map from./// "https://fal.media/files/kangaroo/eNSkRdVFzNvDkrrMjxFA3.png"
    pub control_image_url: String,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    pub image_size: Option<ImageSizeProperty>,
    /// The number of images to generate.
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "A pink owl."
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    pub safety_tolerance: Option<String>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToMusicInput {
    /// The genres (separated by a space ' ') to guide the music generation./// The genres (separated by a space ' ') to guide the music generation./// "inspiring female uplifting pop airy vocal electronic bright vocal vocal"
    /// "R&B male hiphop pop 80s vocal electronic dark vocal vocal"
    pub genres: String,
    /// The prompt to generate an image from. Must have two sections. Sections start with either [chorus] or a [verse]./// The prompt to generate an image from. Must have two sections. Sections start with either [chorus] or a [verse]./// "[verse]\nStaring at the sunset, colors paint the sky\nThoughts of you keep swirling, can't deny\nI know I let you down, I made mistakes\nBut I'm here to mend the heart I didn't break\n\n[chorus]\nEvery road you take, I'll be one step behind\nEvery dream you chase, I'm reaching for the light\nYou can't fight this feeling now\nI won't back down\nYou know you can't deny it now\nI won't back down\n"
    pub lyrics: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProOutpaintInput {
    /// Pixels to expand at the bottom
    pub expand_bottom: Option<i64>,
    /// Pixels to expand on the left
    pub expand_left: Option<i64>,
    /// Pixels to expand on the right
    pub expand_right: Option<i64>,
    /// Pixels to expand at the top
    pub expand_top: Option<i64>,
    /// The image URL to expand using outpainting/// The image URL to expand using outpainting/// "https://storage.googleapis.com/falserverless/flux-lora/example-images/knight.jpeg"
    pub image_url: String,
    /// The number of images to generate.
    pub num_images: Option<i64>,
    /// The format of the generated image.
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    pub safety_tolerance: Option<String>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProCannyControlFinetunedInput {
    /// The control image URL to generate the Canny edge map from./// The control image URL to generate the Canny edge map from./// "https://fal.media/files/kangaroo/eNSkRdVFzNvDkrrMjxFA3.png"
    pub control_image_url: String,
    /// References your specific model
    pub finetune_id: String,
    /// Controls finetune influence.
    /// Increase this value if your target concept isn't showing up strongly enough.
    /// The optimal setting depends on your finetune and prompt
    pub finetune_strength: f64,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    pub image_size: Option<ImageSizeProperty>,
    /// The number of images to generate.
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "A pink owl."
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    pub safety_tolerance: Option<String>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProTextToImageInput {
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    pub image_size: Option<ImageSizeProperty>,
    /// The number of images to generate.
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    pub safety_tolerance: Option<String>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProTextToImageFinetunedInput {
    /// References your specific model
    pub finetune_id: String,
    /// Controls finetune influence.
    /// Increase this value if your target concept isn't showing up strongly enough.
    /// The optimal setting depends on your finetune and prompt
    pub finetune_strength: f64,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    pub image_size: Option<ImageSizeProperty>,
    /// The number of images to generate.
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    pub safety_tolerance: Option<String>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Output {
    /// Whether the generated images contain NSFW concepts.
    pub has_nsfw_concepts: Vec<bool>,
    /// The generated image files info.
    pub images: Vec<Image>,
    /// The prompt used for generating the image.
    pub prompt: String,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
    pub timings: Timings,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProUltraTextToImageInput {
    /// The aspect ratio of the generated image.
    pub aspect_ratio: Option<AspectRatioProperty>,
    /// If set to true, the safety checker will be enabled.
    pub enable_safety_checker: Option<bool>,
    /// The number of images to generate.
    pub num_images: Option<i64>,
    /// The format of the generated image.
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
    pub prompt: String,
    /// Generate less processed, more natural-looking images.
    pub raw: Option<bool>,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    pub safety_tolerance: Option<String>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProDepthControlInput {
    /// The control image URL to generate the depth map from./// The control image URL to generate the depth map from./// "https://fal.media/files/penguin/vt-SeIOweN7_oYBsvGO6t.png"
    pub control_image_url: String,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    pub image_size: Option<ImageSizeProperty>,
    /// The number of images to generate.
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "A blackhole in space."
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    pub safety_tolerance: Option<String>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HTTPValidationError {
    pub detail: Option<Vec<Option<ValidationError>>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum AspectRatioProperty {
    #[serde(rename = "21:9")]
    Property_21_9,
    #[serde(rename = "16:9")]
    Property_16_9,
    #[serde(rename = "4:3")]
    Property_4_3,
    #[serde(rename = "3:2")]
    Property_3_2,
    #[serde(rename = "1:1")]
    Property_1_1,
    #[serde(rename = "2:3")]
    Property_2_3,
    #[serde(rename = "3:4")]
    Property_3_4,
    #[serde(rename = "9:16")]
    Property_9_16,
    #[serde(rename = "9:21")]
    Property_9_21,
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum ImageSizeProperty {
    ImageSize(ImageSize),
    #[serde(rename = "square_hd")]
    SquareHd,
    #[serde(rename = "square")]
    Square,
    #[serde(rename = "portrait_4_3")]
    Portrait43,
    #[serde(rename = "portrait_16_9")]
    Portrait169,
    #[serde(rename = "landscape_4_3")]
    Landscape43,
    #[serde(rename = "landscape_16_9")]
    Landscape169,
}
