#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LCMOutput {
        /// The generated image files info.
pub images: Vec<Image>,
/// A list of booleans indicating whether the generated image contains any
/// potentially unsafe content. If the safety check is disabled, this field
/// will have a false for each generated image.
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
    

                /// Optimized Latent Consistency (SDv1.5)
/// 
/// Produce high-quality images with minimal inference steps. Optimized for 512x512 input image size.
/// 
/// Category: image-to-image
/// Machine Type: A100
                pub fn lcm_sd15_i2i(params: LCMI2IInput) -> FalRequest<LCMI2IInput, LCMOutput> {
                    FalRequest::new(
                        "fal-ai/lcm-sd15-i2i",
                        params
                    )
                }
                