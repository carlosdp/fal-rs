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
                /// The generated video
                ///
                /// Examples:
                ///
                /// {"url":"https://v3.fal.media/files/rabbit/6gJV-z7RJsF0AxkZHkdgJ_output.mp4"}
                pub video: File,
            }

            /// sync.so -- lipsync 1.9.0-beta
            ///
            /// Generate realistic lipsync animations from audio using advanced algorithms for high-quality synchronization.
            ///
            /// Category: video-to-video
            ///
            /// License Type: commercial
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
                /// The generated music
                ///
                /// Examples:
                ///
                /// {"url":"https://fal.media/files/elephant/N5UNLCwkC2y8v7a3LQLFE_output.mp3"}
                pub audio: File,
            }

            /// MiniMax (Hailuo AI) Music
            ///
            /// Generate music from text prompts using the MiniMax model, which leverages advanced AI techniques to create high-quality, diverse musical compositions.
            ///
            /// Category: text-to-audio
            ///
            /// License Type: commercial
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
                /// Generated music file.
                ///
                /// Examples:
                ///
                /// {"content_type":"application/octet-stream","file_name":"output.wav","file_size":33554520,"url":"https://v3.fal.media/files/elephant/VV4wtKXBpZL1bNv6en36t_output.wav"}
                pub audio: File,
            }

            /// DiffRhythm: Lyrics to Song
            ///
            /// DiffRhythm is a blazing fast model for transforming lyrics into full songs. It boasts the capability to generate full songs in less than 30 seconds.
            ///
            /// Category: text-to-audio
            ///
            /// License Type: commercial
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
                /// Generated music file.
                ///
                /// Examples:
                ///
                /// {"content_type":"application/octet-stream","file_name":"cot_inspiring-female-uplifting-pop-airy-vocal-electronic-bright-vocal-vocal_tp0@93_T1@0_rp1@2_maxtk3000_mixed_8179e8da-5452-4cf6-9d6b-f69280feb7e8.mp3","file_size":480462,"url":"https://v3.fal.media/files/tiger/iAXHU3LtbJGeqPYWKkYMr_cot_inspiring-female-uplifting-pop-airy-vocal-electronic-bright-vocal-vocal_tp0%4093_T1%400_rp1%402_maxtk3000_mixed_74bcf408-eb99-4b88-b7bf-7d7212200cf1.mp3"}
                pub audio: File,
            }

            /// YuE: Lyrics to Song
            ///
            /// YuE is a groundbreaking series of open-source foundation models designed for music generation, specifically for transforming lyrics into full songs.
            ///
            /// Category: text-to-audio
            ///
            /// License Type: commercial
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

            /// FLUX1.1 [pro] ultra
            ///
            /// FLUX1.1 [pro] ultra is the newest version of FLUX1.1 [pro], maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
            ///
            /// Category: text-to-image
            /// Machine Type: H100
            /// License Type: commercial
            ///
            /// FLUX.1 [pro], next generation text-to-image model.
            ///
            /// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
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

            /// FLUX1.1 [pro] ultra
            ///
            /// FLUX1.1 [pro] ultra is the newest version of FLUX1.1 [pro], maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
            ///
            /// Category: text-to-image
            /// Machine Type: H100
            /// License Type: commercial
            ///
            /// FLUX.1 [pro], next generation text-to-image model.
            ///
            /// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
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

            /// FLUX1.1 [pro] ultra
            ///
            /// FLUX1.1 [pro] ultra is the newest version of FLUX1.1 [pro], maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
            ///
            /// Category: text-to-image
            /// Machine Type: H100
            /// License Type: commercial
            ///
            /// FLUX.1 [pro], next generation text-to-image model.
            ///
            /// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
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

            /// FLUX1.1 [pro] ultra
            ///
            /// FLUX1.1 [pro] ultra is the newest version of FLUX1.1 [pro], maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
            ///
            /// Category: text-to-image
            /// Machine Type: H100
            /// License Type: commercial
            ///
            /// FLUX1.1 [pro], next generation text-to-image model, with 10x accelerated speeds.
            ///
            /// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
            pub fn v1_1(
                params: FluxProPlusTextToImageInput,
            ) -> FalRequest<FluxProPlusTextToImageInput, Output> {
                FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
            }

            pub mod redux {
                use super::*;

                #[derive(Debug, Serialize, Deserialize)]
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

                /// FLUX1.1 [pro] ultra
                ///
                /// FLUX1.1 [pro] ultra is the newest version of FLUX1.1 [pro], maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
                ///
                /// Category: text-to-image
                /// Machine Type: H100
                /// License Type: commercial
                ///
                /// FLUX 1.1 Redux [pro] API, next generation text-to-image model.
                ///
                /// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
                pub fn redux(params: FluxProRedux) -> FalRequest<FluxProRedux, Output> {
                    FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
                }
            }
        }

        pub mod v1_1_ultra {
            use super::*;

            #[derive(Debug, Serialize, Deserialize)]
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

            /// FLUX1.1 [pro] ultra
            ///
            /// FLUX1.1 [pro] ultra is the newest version of FLUX1.1 [pro], maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
            ///
            /// Category: text-to-image
            /// Machine Type: H100
            /// License Type: commercial
            ///
            /// FLUX1.1 [pro], next generation text-to-image model, with 10x accelerated speeds.
            ///
            /// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
            pub fn v1_1_ultra(
                params: FluxProUltraTextToImageInput,
            ) -> FalRequest<FluxProUltraTextToImageInput, Output> {
                FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
            }

            pub mod redux {
                use super::*;

                #[derive(Debug, Serialize, Deserialize)]
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

                /// FLUX1.1 [pro] ultra
                ///
                /// FLUX1.1 [pro] ultra is the newest version of FLUX1.1 [pro], maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
                ///
                /// Category: text-to-image
                /// Machine Type: H100
                /// License Type: commercial
                ///
                /// FLUX 1.1 Ultra Redux [pro] API, next generation text-to-image model.
                ///
                /// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
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

            /// FLUX1.1 [pro] ultra
            ///
            /// FLUX1.1 [pro] ultra is the newest version of FLUX1.1 [pro], maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
            ///
            /// Category: text-to-image
            /// Machine Type: H100
            /// License Type: commercial
            ///
            /// FLUX1.1 [pro], next generation text-to-image model, with 10x accelerated speeds.
            ///
            /// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
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

                /// FLUX1.1 [pro] ultra
                ///
                /// FLUX1.1 [pro] ultra is the newest version of FLUX1.1 [pro], maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
                ///
                /// Category: text-to-image
                /// Machine Type: H100
                /// License Type: commercial
                ///
                /// FLUX.1 Canny Control [pro] API, next generation text-to-image model.
                ///
                /// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
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

                /// FLUX1.1 [pro] ultra
                ///
                /// FLUX1.1 [pro] ultra is the newest version of FLUX1.1 [pro], maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
                ///
                /// Category: text-to-image
                /// Machine Type: H100
                /// License Type: commercial
                ///
                /// FLUX.1 Canny Control [pro] API, next generation text-to-image model.
                ///
                /// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
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

                /// FLUX1.1 [pro] ultra
                ///
                /// FLUX1.1 [pro] ultra is the newest version of FLUX1.1 [pro], maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
                ///
                /// Category: text-to-image
                /// Machine Type: H100
                /// License Type: commercial
                ///
                /// FLUX.1 Depth Control [pro] API, next generation text-to-image model.
                ///
                /// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
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

                /// FLUX1.1 [pro] ultra
                ///
                /// FLUX1.1 [pro] ultra is the newest version of FLUX1.1 [pro], maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
                ///
                /// Category: text-to-image
                /// Machine Type: H100
                /// License Type: commercial
                ///
                /// FLUX.1 Depth Control [pro] API, next generation text-to-image model.
                ///
                /// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
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

                /// FLUX1.1 [pro] ultra
                ///
                /// FLUX1.1 [pro] ultra is the newest version of FLUX1.1 [pro], maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
                ///
                /// Category: text-to-image
                /// Machine Type: H100
                /// License Type: commercial
                ///
                /// FLUX.1 Fill [pro] API, next generation inpainting/outpainting model.
                ///
                /// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
                pub fn fill(params: FluxProFillInput) -> FalRequest<FluxProFillInput, Output> {
                    FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
                }
            }

            pub mod fill_finetuned {
                use super::*;

                #[derive(Debug, Serialize, Deserialize)]
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

                /// FLUX1.1 [pro] ultra
                ///
                /// FLUX1.1 [pro] ultra is the newest version of FLUX1.1 [pro], maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
                ///
                /// Category: text-to-image
                /// Machine Type: H100
                /// License Type: commercial
                ///
                /// FLUX.1 Fill [pro] API, next generation inpainting/outpainting model.
                ///
                /// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
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

                /// FLUX1.1 [pro] ultra
                ///
                /// FLUX1.1 [pro] ultra is the newest version of FLUX1.1 [pro], maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
                ///
                /// Category: text-to-image
                /// Machine Type: H100
                /// License Type: commercial
                ///
                /// FLUX.1 Outpaint [pro] API, expand images in any direction using outpainting.
                ///
                /// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
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

                /// FLUX1.1 [pro] ultra
                ///
                /// FLUX1.1 [pro] ultra is the newest version of FLUX1.1 [pro], maintaining professional-grade image quality while delivering up to 2K resolution with improved photo realism.
                ///
                /// Category: text-to-image
                /// Machine Type: H100
                /// License Type: commercial
                ///
                /// FLUX.1 Redux [pro] API, next generation text-to-image model.
                ///
                /// All usages of this model must comply with [FLUX.1 PRO Terms of Service](https://blackforestlabs.ai/terms-of-service/).
                pub fn redux(params: FluxProRedux) -> FalRequest<FluxProRedux, Output> {
                    FalRequest::new("fal-ai/flux-pro/v1.1-ultra", params)
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToMusicInput {
    /// The genres (separated by a space ' ') to guide the music generation.
    ///
    /// Examples:
    ///
    /// "inspiring female uplifting pop airy vocal electronic bright vocal vocal"
    /// "R&B male hiphop pop 80s vocal electronic dark vocal vocal"
    pub genres: String,
    /// The prompt to generate an image from. Must have two sections. Sections start with either [chorus] or a [verse].
    ///
    /// Examples:
    ///
    /// "[verse]\nStaring at the sunset, colors paint the sky\nThoughts of you keep swirling, can't deny\nI know I let you down, I made mistakes\nBut I'm here to mend the heart I didn't break\n\n[chorus]\nEvery road you take, I'll be one step behind\nEvery dream you chase, I'm reaching for the light\nYou can't fight this feeling now\nI won't back down\nYou know you can't deny it now\nI won't back down\n"
    pub lyrics: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProCannyControlInput {
    /// The control image URL to generate the Canny edge map from.
    ///
    /// Examples:
    ///
    /// "https://fal.media/files/kangaroo/eNSkRdVFzNvDkrrMjxFA3.png"
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
    /// The prompt to generate an image from.
    ///
    /// Examples:
    ///
    /// "A pink owl."
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
pub struct FluxProFillFinetunedInput {
    /// References your specific model
    pub finetune_id: String,
    /// Controls finetune influence.
    /// Increase this value if your target concept isn't showing up strongly enough.
    /// The optimal setting depends on your finetune and prompt
    pub finetune_strength: f64,
    /// The image URL to generate an image from. Needs to match the dimensions of the mask.
    ///
    /// Examples:
    ///
    /// "https://storage.googleapis.com/falserverless/flux-lora/example-images/knight.jpeg"
    pub image_url: String,
    /// The mask URL to inpaint the image. Needs to match the dimensions of the input image.
    ///
    /// Examples:
    ///
    /// "https://storage.googleapis.com/falserverless/flux-lora/example-images/mask_knight.jpeg"
    pub mask_url: String,
    /// The number of images to generate.
    pub num_images: Option<i64>,
    /// The format of the generated image.
    pub output_format: Option<String>,
    /// The prompt to fill the masked part of the image.
    ///
    /// Examples:
    ///
    /// "A knight in shining armour holding a greatshield with \"FAL\" on it"
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
    /// The prompt to generate an image from.
    ///
    /// Examples:
    ///
    /// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
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
pub struct LipSyncInput {
    /// URL of the input audio
    ///
    /// Examples:
    ///
    /// "https://fal.media/files/lion/vyFWygmZsIZlUO4s0nr2n.wav"
    pub audio_url: String,
    /// The model to use for lipsyncing
    pub model: Option<String>,
    /// Lipsync mode when audio and video durations are out of sync.
    pub sync_mode: Option<String>,
    /// URL of the input video
    ///
    /// Examples:
    ///
    /// "https://fal.media/files/koala/8teUPbRRMtAUTORDvqy0l.mp4"
    pub video_url: String,
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
    /// The prompt to generate an image from.
    ///
    /// Examples:
    ///
    /// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
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
pub struct FluxProUltraTextToImageInputRedux {
    /// The aspect ratio of the generated image.
    pub aspect_ratio: Option<AspectRatioProperty>,
    /// If set to true, the safety checker will be enabled.
    pub enable_safety_checker: Option<bool>,
    /// The strength of the image prompt, between 0 and 1.
    pub image_prompt_strength: Option<f64>,
    /// The image URL to generate an image from. Needs to match the dimensions of the mask.
    ///
    /// Examples:
    ///
    /// "https://fal.media/files/kangaroo/acQvq-Kmo2lajkgvcEHdv.png"
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
pub struct FluxProFillInput {
    /// The image URL to generate an image from. Needs to match the dimensions of the mask.
    ///
    /// Examples:
    ///
    /// "https://storage.googleapis.com/falserverless/flux-lora/example-images/knight.jpeg"
    pub image_url: String,
    /// The mask URL to inpaint the image. Needs to match the dimensions of the input image.
    ///
    /// Examples:
    ///
    /// "https://storage.googleapis.com/falserverless/flux-lora/example-images/mask_knight.jpeg"
    pub mask_url: String,
    /// The number of images to generate.
    pub num_images: Option<i64>,
    /// The format of the generated image.
    pub output_format: Option<String>,
    /// The prompt to fill the masked part of the image.
    ///
    /// Examples:
    ///
    /// "A knight in shining armour holding a greatshield with \"FAL\" on it"
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
pub struct FluxProOutpaintInput {
    /// Pixels to expand at the bottom
    pub expand_bottom: Option<i64>,
    /// Pixels to expand on the left
    pub expand_left: Option<i64>,
    /// Pixels to expand on the right
    pub expand_right: Option<i64>,
    /// Pixels to expand at the top
    pub expand_top: Option<i64>,
    /// The image URL to expand using outpainting
    ///
    /// Examples:
    ///
    /// "https://storage.googleapis.com/falserverless/flux-lora/example-images/knight.jpeg"
    pub image_url: String,
    /// The number of images to generate.
    pub num_images: Option<i64>,
    /// The format of the generated image.
    pub output_format: Option<String>,
    /// The prompt to generate an image from.
    ///
    /// Examples:
    ///
    /// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
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
pub struct TextToMusicRequest {
    /// Lyrics with optional formatting. You can use a newline to separate each line of lyrics. You can use two newlines to add a pause between lines. You can use double hash marks (##) at the beginning and end of the lyrics to add accompaniment. Maximum 600 characters.
    ///
    /// Examples:
    ///
    /// "## Fast and Limitless   \n In the heart of the code, where dreams collide,   \n\nFAL's the name, taking tech for a ride.    \nGenerative media, blazing the trail,   \n\nFast inference power, we'll never fail.\n##"
    pub prompt: String,
    /// Reference song, should contain music and vocals. Must be a .wav or .mp3 file longer than 15 seconds.
    ///
    /// Examples:
    ///
    /// "https://fal.media/files/lion/OOTBTSlxKMH_E8H6hoSlb.mpga"
    pub reference_audio_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProCannyControlFinetunedInput {
    /// The control image URL to generate the Canny edge map from.
    ///
    /// Examples:
    ///
    /// "https://fal.media/files/kangaroo/eNSkRdVFzNvDkrrMjxFA3.png"
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
    /// The prompt to generate an image from.
    ///
    /// Examples:
    ///
    /// "A pink owl."
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
pub struct FluxProUltraTextToImageInput {
    /// The aspect ratio of the generated image.
    pub aspect_ratio: Option<AspectRatioProperty>,
    /// If set to true, the safety checker will be enabled.
    pub enable_safety_checker: Option<bool>,
    /// The number of images to generate.
    pub num_images: Option<i64>,
    /// The format of the generated image.
    pub output_format: Option<String>,
    /// The prompt to generate an image from.
    ///
    /// Examples:
    ///
    /// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
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
pub struct MusicOutput {
    /// The generated music
    ///
    /// Examples:
    ///
    /// {"url":"https://fal.media/files/elephant/N5UNLCwkC2y8v7a3LQLFE_output.mp3"}
    pub audio: File,
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
pub struct LipSyncOutput {
    /// The generated video
    ///
    /// Examples:
    ///
    /// {"url":"https://v3.fal.media/files/rabbit/6gJV-z7RJsF0AxkZHkdgJ_output.mp4"}
    pub video: File,
}

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
    /// The prompt to generate an image from.
    ///
    /// Examples:
    ///
    /// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
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
pub struct ValidationError {
    pub loc: Vec<serde_json::Value>,
    pub msg: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProDepthControlFinetunedInput {
    /// The control image URL to generate the depth map from.
    ///
    /// Examples:
    ///
    /// "https://fal.media/files/penguin/vt-SeIOweN7_oYBsvGO6t.png"
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
    /// The prompt to generate an image from.
    ///
    /// Examples:
    ///
    /// "A blackhole in space."
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProDepthControlInput {
    /// The control image URL to generate the depth map from.
    ///
    /// Examples:
    ///
    /// "https://fal.media/files/penguin/vt-SeIOweN7_oYBsvGO6t.png"
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
    /// The prompt to generate an image from.
    ///
    /// Examples:
    ///
    /// "A blackhole in space."
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
    /// The image URL to generate an image from. Needs to match the dimensions of the mask.
    ///
    /// Examples:
    ///
    /// "https://fal.media/files/kangaroo/acQvq-Kmo2lajkgvcEHdv.png"
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
    /// The prompt to generate an image from.
    ///
    /// Examples:
    ///
    /// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
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
