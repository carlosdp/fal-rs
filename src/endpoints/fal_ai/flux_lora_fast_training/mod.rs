#[allow(unused_imports)]
use crate::prelude::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// URL to the training configuration file.
    pub config_file: File,
    /// URL to the preprocessed images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_preprocessed_output: Option<Option<File>>,
    /// URL to the trained diffusers lora weights.
    pub diffusers_lora_file: File,
}

/// Train Flux LoRA
///
/// Category: training
/// Machine Type: A100
///
///
/// FLUX LoRA fine-tuning endpoint.
///
/// This endpoint fine-tunes a LoRA model on a dataset of images.
/// By default the fine-tuning process is configured for preprocessing a subject.
/// The training will generate both segmentation masks and caption for training.
///
/// If the `is_style` flag is set to `True`,
/// the training a style LoRA, which disables auto-captioning and sengmentation.
///
/// To provide your own captions, you can include a text file with the same name as
/// the image file. For example, if you have an image `photo.jpg`, you can include a
/// text file `photo.txt` with the caption.
///
/// Additionally you can include your own masks for the images. If you have a image
/// `photo.jpg`, you can include a mask file `photo_mask.jpg`.
pub fn flux_lora_fast_training(params: PublicInput) -> FalRequest<PublicInput, Output> {
    FalRequest::new("fal-ai/flux-lora-fast-training", params)
}
