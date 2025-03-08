use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub mod fal_ai {
    use super::*;

    pub mod sync_lipsync {
        use super::*;

        pub mod default {
            use super::*;

            #[derive(Debug, Serialize, Deserialize)]
            pub struct LipSyncOutput {
                pub video: File,
            }

            pub fn default(params: LipSyncInput) -> FalRequest<LipSyncInput, LipSyncOutput> {
                FalRequest::new("fal-ai/sync-lipsync", params)
            }
        }
    }

    pub mod minimax_music {
        use super::*;

        pub mod default {
            use super::*;

            #[derive(Debug, Serialize, Deserialize)]
            pub struct MusicOutput {
                pub audio: File,
            }

            pub fn default(
                params: TextToMusicRequest,
            ) -> FalRequest<TextToMusicRequest, MusicOutput> {
                FalRequest::new("fal-ai/minimax-music", params)
            }
        }
    }

    pub mod diffrhythm {
        use super::*;

        pub mod default {
            use super::*;

            #[derive(Debug, Serialize, Deserialize)]
            pub struct Output {
                pub audio: File,
            }

            pub fn default(params: TextToMusicInput) -> FalRequest<TextToMusicInput, Output> {
                FalRequest::new("fal-ai/diffrhythm", params)
            }
        }
    }

    pub mod yue {
        use super::*;

        pub mod default {
            use super::*;

            #[derive(Debug, Serialize, Deserialize)]
            pub struct Output {
                pub audio: File,
            }

            pub fn default(params: TextToMusicInput) -> FalRequest<TextToMusicInput, Output> {
                FalRequest::new("fal-ai/yue", params)
            }
        }
    }

    pub mod flux_pro {
        use super::*;

        pub mod default {
            use super::*;

            #[derive(Debug, Serialize, Deserialize)]
            pub struct Output {
                pub has_nsfw_concepts: Vec<bool>,
                pub images: Vec<Image>,
                pub prompt: String,
                pub seed: i64,
                pub timings: Timings,
            }

            pub fn default(
                params: FluxProTextToImageInput,
            ) -> FalRequest<FluxProTextToImageInput, Output> {
                FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
            }
        }

        pub mod finetuned {
            use super::*;

            #[derive(Debug, Serialize, Deserialize)]
            pub struct Output {
                pub has_nsfw_concepts: Vec<bool>,
                pub images: Vec<Image>,
                pub prompt: String,
                pub seed: i64,
                pub timings: Timings,
            }

            pub fn finetuned(
                params: FluxProTextToImageFinetunedInput,
            ) -> FalRequest<FluxProTextToImageFinetunedInput, Output> {
                FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
            }
        }

        pub mod new {
            use super::*;

            #[derive(Debug, Serialize, Deserialize)]
            pub struct Output {
                pub has_nsfw_concepts: Vec<bool>,
                pub images: Vec<Image>,
                pub prompt: String,
                pub seed: i64,
                pub timings: Timings,
            }

            pub fn new(
                params: FluxProTextToImageInput,
            ) -> FalRequest<FluxProTextToImageInput, Output> {
                FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
            }
        }

        pub mod v1_1 {
            use super::*;

            #[derive(Debug, Serialize, Deserialize)]
            pub struct Output {
                pub has_nsfw_concepts: Vec<bool>,
                pub images: Vec<Image>,
                pub prompt: String,
                pub seed: i64,
                pub timings: Timings,
            }

            pub fn v1_1(
                params: FluxProPlusTextToImageInput,
            ) -> FalRequest<FluxProPlusTextToImageInput, Output> {
                FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
            }

            pub mod redux {
                use super::*;

                #[derive(Debug, Serialize, Deserialize)]
                pub struct Output {
                    pub has_nsfw_concepts: Vec<bool>,
                    pub images: Vec<Image>,
                    pub prompt: String,
                    pub seed: i64,
                    pub timings: Timings,
                }

                pub fn redux(params: FluxProRedux) -> FalRequest<FluxProRedux, Output> {
                    FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
                }
            }
        }

        pub mod v1_1_ultra {
            use super::*;

            #[derive(Debug, Serialize, Deserialize)]
            pub struct Output {
                pub has_nsfw_concepts: Vec<bool>,
                pub images: Vec<Image>,
                pub prompt: String,
                pub seed: i64,
                pub timings: Timings,
            }

            pub fn v1_1_ultra(
                params: FluxProUltraTextToImageInput,
            ) -> FalRequest<FluxProUltraTextToImageInput, Output> {
                FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
            }

            pub mod redux {
                use super::*;

                #[derive(Debug, Serialize, Deserialize)]
                pub struct Output {
                    pub has_nsfw_concepts: Vec<bool>,
                    pub images: Vec<Image>,
                    pub prompt: String,
                    pub seed: i64,
                    pub timings: Timings,
                }

                pub fn redux(
                    params: FluxProUltraTextToImageInputRedux,
                ) -> FalRequest<FluxProUltraTextToImageInputRedux, Output> {
                    FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
                }
            }
        }

        pub mod v1_1_ultra_finetuned {
            use super::*;

            #[derive(Debug, Serialize, Deserialize)]
            pub struct Output {
                pub has_nsfw_concepts: Vec<bool>,
                pub images: Vec<Image>,
                pub prompt: String,
                pub seed: i64,
                pub timings: Timings,
            }

            pub fn v1_1_ultra_finetuned(
                params: FluxProUltraTextToImageFinetunedInput,
            ) -> FalRequest<FluxProUltraTextToImageFinetunedInput, Output> {
                FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
            }
        }

        pub mod v1 {
            use super::*;

            pub mod canny {
                use super::*;

                #[derive(Debug, Serialize, Deserialize)]
                pub struct Output {
                    pub has_nsfw_concepts: Vec<bool>,
                    pub images: Vec<Image>,
                    pub prompt: String,
                    pub seed: i64,
                    pub timings: Timings,
                }

                pub fn canny(
                    params: FluxProCannyControlInput,
                ) -> FalRequest<FluxProCannyControlInput, Output> {
                    FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
                }
            }

            pub mod canny_finetuned {
                use super::*;

                #[derive(Debug, Serialize, Deserialize)]
                pub struct Output {
                    pub has_nsfw_concepts: Vec<bool>,
                    pub images: Vec<Image>,
                    pub prompt: String,
                    pub seed: i64,
                    pub timings: Timings,
                }

                pub fn canny_finetuned(
                    params: FluxProCannyControlFinetunedInput,
                ) -> FalRequest<FluxProCannyControlFinetunedInput, Output> {
                    FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
                }
            }

            pub mod depth {
                use super::*;

                #[derive(Debug, Serialize, Deserialize)]
                pub struct Output {
                    pub has_nsfw_concepts: Vec<bool>,
                    pub images: Vec<Image>,
                    pub prompt: String,
                    pub seed: i64,
                    pub timings: Timings,
                }

                pub fn depth(
                    params: FluxProDepthControlInput,
                ) -> FalRequest<FluxProDepthControlInput, Output> {
                    FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
                }
            }

            pub mod depth_finetuned {
                use super::*;

                #[derive(Debug, Serialize, Deserialize)]
                pub struct Output {
                    pub has_nsfw_concepts: Vec<bool>,
                    pub images: Vec<Image>,
                    pub prompt: String,
                    pub seed: i64,
                    pub timings: Timings,
                }

                pub fn depth_finetuned(
                    params: FluxProDepthControlFinetunedInput,
                ) -> FalRequest<FluxProDepthControlFinetunedInput, Output> {
                    FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
                }
            }

            pub mod fill {
                use super::*;

                #[derive(Debug, Serialize, Deserialize)]
                pub struct Output {
                    pub has_nsfw_concepts: Vec<bool>,
                    pub images: Vec<Image>,
                    pub prompt: String,
                    pub seed: i64,
                    pub timings: Timings,
                }

                pub fn fill(params: FluxProFillInput) -> FalRequest<FluxProFillInput, Output> {
                    FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
                }
            }

            pub mod fill_finetuned {
                use super::*;

                #[derive(Debug, Serialize, Deserialize)]
                pub struct Output {
                    pub has_nsfw_concepts: Vec<bool>,
                    pub images: Vec<Image>,
                    pub prompt: String,
                    pub seed: i64,
                    pub timings: Timings,
                }

                pub fn fill_finetuned(
                    params: FluxProFillFinetunedInput,
                ) -> FalRequest<FluxProFillFinetunedInput, Output> {
                    FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
                }
            }

            pub mod outpaint {
                use super::*;

                #[derive(Debug, Serialize, Deserialize)]
                pub struct Output {
                    pub has_nsfw_concepts: Vec<bool>,
                    pub images: Vec<Image>,
                    pub prompt: String,
                    pub seed: i64,
                    pub timings: Timings,
                }

                pub fn outpaint(
                    params: FluxProOutpaintInput,
                ) -> FalRequest<FluxProOutpaintInput, Output> {
                    FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
                }
            }

            pub mod redux {
                use super::*;

                #[derive(Debug, Serialize, Deserialize)]
                pub struct Output {
                    pub has_nsfw_concepts: Vec<bool>,
                    pub images: Vec<Image>,
                    pub prompt: String,
                    pub seed: i64,
                    pub timings: Timings,
                }

                pub fn redux(params: FluxProRedux) -> FalRequest<FluxProRedux, Output> {
                    FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProFillInput {
    pub image_url: String,
    pub mask_url: String,
    pub num_images: Option<i64>,
    pub output_format: Option<String>,
    pub prompt: String,
    pub safety_tolerance: Option<String>,
    pub seed: Option<i64>,
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProRedux {
    pub guidance_scale: Option<f64>,
    pub image_size: Option<ImageSizeProperty>,
    pub image_url: String,
    pub num_images: Option<i64>,
    pub num_inference_steps: Option<i64>,
    pub output_format: Option<String>,
    pub prompt: Option<String>,
    pub safety_tolerance: Option<String>,
    pub seed: Option<i64>,
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LipSyncInput {
    pub audio_url: String,
    pub model: Option<String>,
    pub sync_mode: Option<String>,
    pub video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProCannyControlFinetunedInput {
    pub control_image_url: String,
    pub finetune_id: String,
    pub finetune_strength: f64,
    pub guidance_scale: Option<f64>,
    pub image_size: Option<ImageSizeProperty>,
    pub num_images: Option<i64>,
    pub num_inference_steps: Option<i64>,
    pub output_format: Option<String>,
    pub prompt: String,
    pub safety_tolerance: Option<String>,
    pub seed: Option<i64>,
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProOutpaintInput {
    pub expand_bottom: Option<i64>,
    pub expand_left: Option<i64>,
    pub expand_right: Option<i64>,
    pub expand_top: Option<i64>,
    pub image_url: String,
    pub num_images: Option<i64>,
    pub output_format: Option<String>,
    pub prompt: String,
    pub safety_tolerance: Option<String>,
    pub seed: Option<i64>,
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProTextToImageInput {
    pub guidance_scale: Option<f64>,
    pub image_size: Option<ImageSizeProperty>,
    pub num_images: Option<i64>,
    pub num_inference_steps: Option<i64>,
    pub output_format: Option<String>,
    pub prompt: String,
    pub safety_tolerance: Option<String>,
    pub seed: Option<i64>,
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MusicOutput {
    pub audio: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProCannyControlInput {
    pub control_image_url: String,
    pub guidance_scale: Option<f64>,
    pub image_size: Option<ImageSizeProperty>,
    pub num_images: Option<i64>,
    pub num_inference_steps: Option<i64>,
    pub output_format: Option<String>,
    pub prompt: String,
    pub safety_tolerance: Option<String>,
    pub seed: Option<i64>,
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProUltraTextToImageInputRedux {
    pub aspect_ratio: Option<AspectRatioProperty>,
    pub enable_safety_checker: Option<bool>,
    pub image_prompt_strength: Option<f64>,
    pub image_url: String,
    pub num_images: Option<i64>,
    pub output_format: Option<String>,
    pub prompt: Option<String>,
    pub raw: Option<bool>,
    pub safety_tolerance: Option<String>,
    pub seed: Option<i64>,
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProPlusTextToImageInput {
    pub enable_safety_checker: Option<bool>,
    pub image_size: Option<ImageSizeProperty>,
    pub num_images: Option<i64>,
    pub output_format: Option<String>,
    pub prompt: String,
    pub safety_tolerance: Option<String>,
    pub seed: Option<i64>,
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Output {
    pub has_nsfw_concepts: Vec<bool>,
    pub images: Vec<Image>,
    pub prompt: String,
    pub seed: i64,
    pub timings: Timings,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ValidationError {
    pub loc: Vec<serde_json::Value>,
    pub msg: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageSize {
    pub height: Option<i64>,
    pub width: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LipSyncOutput {
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToMusicInput {
    pub genres: String,
    pub lyrics: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProFillFinetunedInput {
    pub finetune_id: String,
    pub finetune_strength: f64,
    pub image_url: String,
    pub mask_url: String,
    pub num_images: Option<i64>,
    pub output_format: Option<String>,
    pub prompt: String,
    pub safety_tolerance: Option<String>,
    pub seed: Option<i64>,
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProUltraTextToImageFinetunedInput {
    pub aspect_ratio: Option<AspectRatioProperty>,
    pub enable_safety_checker: Option<bool>,
    pub finetune_id: String,
    pub finetune_strength: f64,
    pub num_images: Option<i64>,
    pub output_format: Option<String>,
    pub prompt: String,
    pub raw: Option<bool>,
    pub safety_tolerance: Option<String>,
    pub seed: Option<i64>,
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToMusicRequest {
    pub prompt: String,
    pub reference_audio_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProTextToImageFinetunedInput {
    pub finetune_id: String,
    pub finetune_strength: f64,
    pub guidance_scale: Option<f64>,
    pub image_size: Option<ImageSizeProperty>,
    pub num_images: Option<i64>,
    pub num_inference_steps: Option<i64>,
    pub output_format: Option<String>,
    pub prompt: String,
    pub safety_tolerance: Option<String>,
    pub seed: Option<i64>,
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProDepthControlFinetunedInput {
    pub control_image_url: String,
    pub finetune_id: String,
    pub finetune_strength: f64,
    pub guidance_scale: Option<f64>,
    pub image_size: Option<ImageSizeProperty>,
    pub num_images: Option<i64>,
    pub num_inference_steps: Option<i64>,
    pub output_format: Option<String>,
    pub prompt: String,
    pub safety_tolerance: Option<String>,
    pub seed: Option<i64>,
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HTTPValidationError {
    pub detail: Option<Vec<Option<ValidationError>>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProDepthControlInput {
    pub control_image_url: String,
    pub guidance_scale: Option<f64>,
    pub image_size: Option<ImageSizeProperty>,
    pub num_images: Option<i64>,
    pub num_inference_steps: Option<i64>,
    pub output_format: Option<String>,
    pub prompt: String,
    pub safety_tolerance: Option<String>,
    pub seed: Option<i64>,
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProUltraTextToImageInput {
    pub aspect_ratio: Option<AspectRatioProperty>,
    pub enable_safety_checker: Option<bool>,
    pub num_images: Option<i64>,
    pub output_format: Option<String>,
    pub prompt: String,
    pub raw: Option<bool>,
    pub safety_tolerance: Option<String>,
    pub seed: Option<i64>,
    pub sync_mode: Option<bool>,
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
