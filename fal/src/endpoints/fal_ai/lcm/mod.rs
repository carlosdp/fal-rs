#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LCMOutput {
        /// The generated image files info.
pub images: Vec<Image>,
/// A list of booleans indicating whether the generated image contains any
/// potentially unsafe content. If the safety check is disabled, this field
/// will all will be false.
pub nsfw_content_detected: Vec<bool>,
/// Number of inference steps used to generate the image. It will be the same value of the one passed in the
/// input or the default one in case none was passed.
#[serde(skip_serializing_if = "Option::is_none")]
pub num_inference_steps: Option<i64>,
/// An id bound to a request, can be used with response to identify the request
/// itself.
#[serde(skip_serializing_if = "Option::is_none")]
pub request_id: Option<String>,
/// Seed of the generated Image. It will be the same value of the one passed in the
/// input or the randomly generated that was used in case none was passed.
pub seed: i64,
pub timings: Timings
    }
    

                /// Latent Consistency (SDXL & SDv1.5)
/// 
/// Produce high-quality images with minimal inference steps.
/// 
/// Category: text-to-image
/// Machine Type: A100
/// 
/// 
/// Generates an image using the given prompt and model.
/// 
/// If only the prompt is given, the model will generate an image from scratch, this
/// process is also known as "text-to-image".
/// 
/// If an image is given (via `image_url` parameter), the model will generate an image
/// that's similar to the given image depending on the `strength` parameter. This
/// process is also known as "image-to-image".
/// 
/// If an image and a mask are given (via `image_url` and `mask_url` parameters), the
/// model will generate an image that fills the mask area with the most relevant
/// content from the given image. This process is also known as "inpainting".
                pub fn lcm(params: LCMInput) -> FalRequest<LCMInput, LCMOutput> {
                    FalRequest::new(
                        "fal-ai/lcm",
                        params
                    )
                }
                