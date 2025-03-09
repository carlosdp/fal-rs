use std::collections::HashMap;

use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CannyInput {
    /// High threshold for the hysteresis procedure. Edges with a strength higher than the high threshold will always appear as edges in the output image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_threshold: Option<i64>,
    /// URL of the image to process/// URL of the image to process/// "https://storage.googleapis.com/falserverless/model_tests/image_preprocessors/cat.png"
    pub image_url: String,
    /// Low threshold for the hysteresis procedure. Edges with a strength higher than the low threshold will appear in the output image, if there are strong edges nearby.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_threshold: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RemoveBackgroundOutput {
    /// Background removed image.
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GrowMaskOutput {
    /// The mask/// The mask/// {"content_type":"image/png","height":700,"url":"https://storage.googleapis.com/falserverless/model_tests/workflow_utils/grow_output.png","width":610}
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EraserOutput {
    /// The generated image/// The generated image/// {"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_eraser_res.png"}
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BrPortugueseRequest {
    pub prompt: String,
    /// Voice ID for the desired voice./// Voice ID for the desired voice./// "pf_dora"
    pub voice: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BrEnglishRequest {
    pub prompt: String,
    /// Voice ID for the desired voice./// Voice ID for the desired voice./// "bf_alice"
    pub voice: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LDMVoiceInput {
    /// A prefix to identify the speaker in multi-turn dialogues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_prefix: Option<String>,
    /// The unique ID of a PlayHT or Cloned Voice, or a name from the available presets./// The unique ID of a PlayHT or Cloned Voice, or a name from the available presets./// "Jennifer (English (US)/American)"
    /// "Dexter (English (US)/American)"
    /// "Ava (English (AU)/Australian)"
    /// "Tilly (English (AU)/Australian)"
    /// "Charlotte (Advertising) (English (CA)/Canadian)"
    /// "Charlotte (Meditation) (English (CA)/Canadian)"
    /// "Cecil (English (GB)/British)"
    /// "Sterling (English (GB)/British)"
    /// "Cillian (English (IE)/Irish)"
    /// "Madison (English (IE)/Irish)"
    /// "Ada (English (ZA)/South african)"
    /// "Furio (English (IT)/Italian)"
    /// "Alessandro (English (IT)/Italian)"
    /// "Carmen (English (MX)/Mexican)"
    /// "Sumita (English (IN)/Indian)"
    /// "Navya (English (IN)/Indian)"
    /// "Baptiste (English (FR)/French)"
    /// "Lumi (English (FI)/Finnish)"
    /// "Ronel Conversational (Afrikaans/South african)"
    /// "Ronel Narrative (Afrikaans/South african)"
    /// "Abdo Conversational (Arabic/Arabic)"
    /// "Abdo Narrative (Arabic/Arabic)"
    /// "Mousmi Conversational (Bengali/Bengali)"
    /// "Mousmi Narrative (Bengali/Bengali)"
    /// "Caroline Conversational (Portuguese (BR)/Brazilian)"
    /// "Caroline Narrative (Portuguese (BR)/Brazilian)"
    /// "Ange Conversational (French/French)"
    /// "Ange Narrative (French/French)"
    /// "Anke Conversational (German/German)"
    /// "Anke Narrative (German/German)"
    /// "Bora Conversational (Greek/Greek)"
    /// "Bora Narrative (Greek/Greek)"
    /// "Anuj Conversational (Hindi/Indian)"
    /// "Anuj Narrative (Hindi/Indian)"
    /// "Alessandro Conversational (Italian/Italian)"
    /// "Alessandro Narrative (Italian/Italian)"
    /// "Kiriko Conversational (Japanese/Japanese)"
    /// "Kiriko Narrative (Japanese/Japanese)"
    /// "Dohee Conversational (Korean/Korean)"
    /// "Dohee Narrative (Korean/Korean)"
    /// "Ignatius Conversational (Malay/Malay)"
    /// "Ignatius Narrative (Malay/Malay)"
    /// "Adam Conversational (Polish/Polish)"
    /// "Adam Narrative (Polish/Polish)"
    /// "Andrei Conversational (Russian/Russian)"
    /// "Andrei Narrative (Russian/Russian)"
    /// "Aleksa Conversational (Serbian/Serbian)"
    /// "Aleksa Narrative (Serbian/Serbian)"
    /// "Carmen Conversational (Spanish/Spanish)"
    /// "Patricia Conversational (Spanish/Spanish)"
    /// "Aiken Conversational (Tagalog/Filipino)"
    /// "Aiken Narrative (Tagalog/Filipino)"
    /// "Katbundit Conversational (Thai/Thai)"
    /// "Katbundit Narrative (Thai/Thai)"
    /// "Ali Conversational (Turkish/Turkish)"
    /// "Ali Narrative (Turkish/Turkish)"
    /// "Sahil Conversational (Urdu/Pakistani)"
    /// "Sahil Narrative (Urdu/Pakistani)"
    /// "Mary Conversational (Hebrew/Israeli)"
    /// "Mary Narrative (Hebrew/Israeli)"
    pub voice: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ResizeImageInput {
    /// Position of cropping. Only used when mode is 'crop', default is center
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cropping_position: Option<String>,
    /// Height of the resized image/// Height of the resized image/// 700
    pub height: i64,
    /// Input image url./// Input image url./// "https://storage.googleapis.com/falserverless/model_tests/workflow_utils/mask_input.png"
    pub image_url: String,
    /// Resizing mode
    pub mode: String,
    /// Color of padding. Only used when mode is 'pad', default is black
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_color: Option<String>,
    /// Resizing strategy. Only used when mode is 'scale', default is nearest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resampling: Option<String>,
    /// Proportions of the image. Only used when mode is 'scale', default is fit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_proportions: Option<String>,
    /// Width of the resized image/// Width of the resized image/// 610
    pub width: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DetectionOutput {
    /// Output image with detection visualization
    pub image: Image,
    /// Detection results as text
    pub text_output: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HunyuanVideoRequest {
    /// The aspect ratio of the video to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// URL of the image input./// URL of the image input./// "https://storage.googleapis.com/falserverless/example_inputs/hunyuan_i2v.jpg"
    pub image_url: String,
    /// The number of frames to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_frames: Option<String>,
    /// The prompt to generate the video from./// The prompt to generate the video from./// "Two muscular cats boxing in a boxing ring."
    pub prompt: String,
    /// The resolution of the video to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// The seed to use for generating the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BoundingBoxes {
    /// List of bounding boxes
    pub bboxes: Vec<BoundingBox>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToVideoOutput {
    /// The generated video/// The generated video/// {"url":"https://v3.fal.media/files/tiger/83-YzufmOlsnhqq5ed382_output.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OutputParameters {
    /// The latents saved for debugging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_latents: Option<Option<File>>,
    /// The latents saved for debugging per pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_per_pass_latents: Option<Option<File>>,
    /// Whether the generated images contain NSFW concepts.
    pub has_nsfw_concepts: Vec<bool>,
    /// The generated image files info.
    pub images: Vec<Image>,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NafnetInputDenoise {
    /// URL of image to be used for relighting/// URL of image to be used for relighting/// "https://storage.googleapis.com/falserverless/nafnet/noisy.png"
    pub image_url: String,
    /// seed to be used for generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TranscriptionOutput {
    /// Detected or specified language code
    pub language_code: String,
    /// Confidence in language detection
    pub language_probability: f64,
    /// The full transcribed text
    pub text: String,
    /// Word-level transcription details
    pub words: Vec<TranscriptionWord>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SamInput {
    /// URL of the image to process/// URL of the image to process/// "https://storage.googleapis.com/falserverless/model_tests/image_preprocessors/cat.png"
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NafnetOutputDenoise {
    /// The generated image file info./// The generated image file info./// {"content_type":"image/png","file_name":"36d3ca4791a647678b2ff01a35c87f5a.png","file_size":423052,"height":512,"url":"https://storage.googleapis.com/falserverless/nafnet/7c97e55956324a7cbee00ac9652a931b.png","width":512}
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AMTInterpolationOutput {
    /// Generated video
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReimagineOutput {
    /// The generated images/// The generated images/// [{"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_reimagine_output.png"}]
    pub images: Vec<Image>,
    /// Seed value used for generation.
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreativeUpscalerOutput {
    /// The generated image file info.
    pub image: Image,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ChatInput {
    /// Name of the model to use. Premium models are charged at 10x the rate of standard models, they include: meta-llama/llama-3.2-90b-vision-instruct, openai/gpt-4o, anthropic/claude-3-5-haiku, google/gemini-pro-1.5, anthropic/claude-3.5-sonnet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Prompt to be used for the chat completion/// Prompt to be used for the chat completion/// "What is the meaning of life?"
    pub prompt: String,
    /// Should reasoning be the part of the final answer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<bool>,
    /// System prompt to provide context or instructions to the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<SystemPromptProperty>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LivePortraitImageInput {
    /// Amount to open mouth in 'aaa' shape
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aaa: Option<f64>,
    /// Amount to blink the eyes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blink: Option<f64>,
    /// Size of the output image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dsize: Option<i64>,
    /// Amount to shape mouth in 'eee' position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eee: Option<f64>,
    /// Whether to enable the safety checker. If enabled, the model will check if the input image contains a face before processing it.
    /// The safety checker will process the input image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// Amount to raise or lower eyebrows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eyebrow: Option<f64>,
    /// Whether to crop the source portrait to the face-cropping space.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_do_crop: Option<bool>,
    /// Whether to conduct the rotation when flag_do_crop is True.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_do_rot: Option<bool>,
    /// Whether to set the lip to closed state before animation. Only takes effect when flag_eye_retargeting and flag_lip_retargeting are False.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_lip_zero: Option<bool>,
    /// Whether to paste-back/stitch the animated face cropping from the face-cropping space to the original image space.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_pasteback: Option<bool>,
    /// URL of the image to be animated/// URL of the image to be animated/// "https://storage.googleapis.com/falserverless/model_tests/live-portrait/XKEmk3mAzGHUjK3qqH-UL.jpeg"
    pub image_url: String,
    /// Output format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// Amount to move pupils horizontally
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pupil_x: Option<f64>,
    /// Amount to move pupils vertically
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pupil_y: Option<f64>,
    /// Amount to rotate the face in pitch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_pitch: Option<f64>,
    /// Amount to rotate the face in roll
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_roll: Option<f64>,
    /// Amount to rotate the face in yaw
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_yaw: Option<f64>,
    /// Scaling factor for the face crop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    /// Amount to smile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smile: Option<f64>,
    /// Horizontal offset ratio for face crop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vx_ratio: Option<f64>,
    /// Vertical offset ratio for face crop. Positive values move up, negative values move down.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vy_ratio: Option<f64>,
    /// Amount to wink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wink: Option<f64>,
    /// Amount to shape mouth in 'woo' position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub woo: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VideoEffectsOutput {
    /// The generated video/// The generated video/// {"content_type":"video/mp4","file_name":"output.mp4","url":"https://storage.googleapis.com/falserverless/kling/kling_ex.mp4.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TransparentImageToMaskOutput {
    /// The mask/// The mask/// {"content_type":"image/png","height":700,"url":"https://storage.googleapis.com/falserverless/model_tests/workflow_utils/transparent_image_to_mask_output.png","width":610}
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BoundingBoxOutputWithLabels {
    /// Processed image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Option<Image>>,
    /// Results from the model
    pub results: BoundingBoxes,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GrowMaskInput {
    /// Input image url./// Input image url./// "https://storage.googleapis.com/falserverless/model_tests/workflow_utils/mask_input.png"
    pub image_url: String,
    /// The number of pixels to grow the mask./// The number of pixels to grow the mask./// 5

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixels: Option<i64>,
    /// The threshold to convert the image to a mask. 0-255./// The threshold to convert the image to a mask. 0-255./// 128

    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReferenceImage {
    /// URL to the reference image file (PNG format recommended)
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageToImageHyperInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://fal-cdn.batuhan-941.workers.dev/files/tiger/IExuP-WICqaIesLZAZPur.jpeg"
    pub image_url: String,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<String>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "an island near sea, with seagulls, moon shining over the sea, light house, boats int he background, fish flying over the sea"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
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
pub struct VideoOutput {
    /// Seed for random number generator
    pub seed: i64,
    /// Generated video
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StyleReferenceOutput {
    /// The ID of the created style, this ID can be used to reference the style in the future.
    pub style_id: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StepFunT2VRequest {
    /// If set to true, the safety checker will be enabled./// If set to true, the safety checker will be enabled./// true

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The number of inference steps to run. Lower gets faster results, higher gets better results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to generate the video from./// The prompt to generate the video from./// "A stylish woman walks down a Tokyo street filled with warm glowing neon and animated city signage. She wears a black leather jacket, a long red dress, and black boots, and carries a black purse."
    pub prompt: String,
    /// The seed to use for generating the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ItalianOutput {
    /// The generated music/// The generated music/// {"url":"https://fal.media/files/monkey/-MZ0hRO4IpTMukb_S5aRZ_tmpin14eoed.wav"}
    pub audio: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Color {
    /// Blue value/// Blue value/// 128

    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<i64>,
    /// Green value/// Green value/// 128

    #[serde(skip_serializing_if = "Option::is_none")]
    pub g: Option<i64>,
    /// Red value/// Red value/// 128

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AnimateDiffV2VTurboInput {
    /// The first N number of seconds of video to animate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_n_seconds: Option<i64>,
    /// Number of frames per second to extract from the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<i64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The motions to apply to the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motions: Option<Vec<Option<String>>>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of inference steps to perform. 4-12 is recommended for turbo mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "closeup of tony stark, robert downey jr, fireworks, high quality, ultra HD"
    /// "panda playing a guitar, on a boat, in the ocean, high quality, high quality, ultra HD, realistic"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The strength of the input video in the final output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// URL of the video./// URL of the video./// "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-vid2vid-input-2.gif"
    /// "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-vid2vid-input-1.gif"
    pub video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MoondreamInputParam {
    /// URL of the image to be processed/// URL of the image to be processed/// "https://llava-vl.github.io/static/images/monalisa.jpg"
    pub image_url: String,
    /// Prompt to be used for the image/// Prompt to be used for the image/// "Do you know who drew this painting?"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SigmasInput {
    /// Sigmas schedule to be used if 'custom' method is selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array: Option<Vec<Option<f64>>>,
    /// The method to use for the sigmas. If set to 'custom', the sigmas will be set based
    /// on the provided sigmas schedule in the `array` field.
    /// Defaults to 'default' which means the scheduler will use the sigmas of the scheduler.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WaveformOutput {
    /// Duration of the audio in seconds
    pub duration: f64,
    /// Number of points in the waveform data
    pub points: i64,
    /// Number of decimal places used in the waveform values
    pub precision: i64,
    /// Normalized waveform data as an array of values between -1 and 1. The number of points is determined by audio duration × points_per_second.
    pub waveform: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DynamicMask {
    /// URL of the image for Dynamic Brush Application Area (Mask image created by users using the motion brush)/// URL of the image for Dynamic Brush Application Area (Mask image created by users using the motion brush)/// "https://h2.inkwai.com/bs2/upload-ylab-stunt/ai_portal/1732888130/WU8spl23dA/dynamic_mask_1.png"
    pub mask_url: String,
    /// List of trajectories/// List of trajectories/// [{"x":279,"y":219},{"x":417,"y":65}]

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trajectories: Option<Vec<Option<Trajectory>>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToImageLightningInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<String>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "photo of a girl smiling during a sunset, with lightnings in the background"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
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
pub struct ImageToImagePlaygroundv25Input {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The rescale factor for the CFG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_rescale: Option<f64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://fal-cdn.batuhan-941.workers.dev/files/tiger/IExuP-WICqaIesLZAZPur.jpeg"
    pub image_url: String,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "an island near sea, with seagulls, moon shining over the sea, light house, boats int he background, fish flying over the sea"
    pub prompt: String,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SadTalkerInput {
    /// URL of the driven audio/// URL of the driven audio/// "https://storage.googleapis.com/falserverless/model_tests/sadtalker/deyu.wav"
    pub driven_audio_url: String,
    /// The scale of the expression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_scale: Option<f64>,
    /// The type of face enhancer to use/// The type of face enhancer to use/// null

    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_enhancer: Option<String>,
    /// The resolution of the face model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_resolution: Option<String>,
    /// The style of the pose
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose_style: Option<i64>,
    /// The type of preprocessing to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preprocess: Option<String>,
    /// URL of the source image/// URL of the source image/// "https://storage.googleapis.com/falserverless/model_tests/sadtalker/anime_girl.png"
    pub source_image_url: String,
    /// Whether to use still mode. Fewer head motion, works with preprocess `full`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub still_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchQueryInput {
    /// List of image URLs to be processed (maximum 32 images)
    pub images_data_url: String,
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    /// Single prompt to apply to all images/// Single prompt to apply to all images/// "Describe this image in detail."
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SwinSrInput {
    /// URL of image to be used for image enhancement/// URL of image to be used for image enhancement/// "https://storage.googleapis.com/falserverless/gallery/seoul.jpg"
    pub image_url: String,
    /// seed to be used for generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Task to perform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LipSyncInput {
    /// URL of the input audio/// URL of the input audio/// "https://fal.media/files/lion/vyFWygmZsIZlUO4s0nr2n.wav"
    pub audio_url: String,
    /// The model to use for lipsyncing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Lipsync mode when audio and video durations are out of sync.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<String>,
    /// URL of the input video/// URL of the input video/// "https://fal.media/files/koala/8teUPbRRMtAUTORDvqy0l.mp4"
    pub video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Audio {
    /// Overall bitrate of the media in bits per second
    pub bitrate: i64,
    /// Number of audio channels
    pub channels: i64,
    /// Codec used to encode the media
    pub codec: String,
    /// Container format of the media file (e.g., 'mp4', 'mov')
    pub container: String,
    /// MIME type of the media file
    pub content_type: String,
    /// Duration of the media in seconds
    pub duration: f64,
    /// Original filename of the media
    pub file_name: String,
    /// Size of the file in bytes
    pub file_size: i64,
    /// Type of media (always 'audio')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// Audio sample rate in Hz
    pub sample_rate: i64,
    /// URL where the media file can be accessed
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MochiT2VOutput {
    /// The generated video/// The generated video/// {"url":"https://fal.media/files/zebra/GScPi-7ma3Fn8r1O1on4z_output_1729631871.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToImageLCMInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The rescale factor for the CFG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_rescale: Option<f64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The name of the model to use./// The name of the model to use./// "stabilityai/stable-diffusion-xl-base-1.0"
    /// "runwayml/stable-diffusion-v1-5"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"
    /// "ugly, deformed"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "Self-portrait oil painting, a beautiful cyborg with golden hair, 8k."
    pub prompt: String,
    /// An id bound to a request, can be used with response to identify the request
    /// itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
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
pub struct CCSRInput {
    /// Type of color correction for samples./// Type of color correction for samples./// "adain"
    /// "wavelet"
    /// "none"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_fix_type: Option<String>,
    /// The text prompt you would like to convert to speech./// The text prompt you would like to convert to speech./// "https://storage.googleapis.com/falserverless/gallery/blue-bird.jpeg"
    pub image_url: String,
    /// The scale of the output image. The higher the scale, the bigger the output image will be.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    /// Seed for reproducibility. Different seeds will make slightly different results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The number of steps to run the model for. The higher the number the better the quality and longer it will take to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<i64>,
    /// The ending point of uniform sampling strategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_max: Option<f64>,
    /// The starting point of uniform sampling strategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_min: Option<f64>,
    /// If specified, a patch-based sampling strategy will be used for sampling./// If specified, a patch-based sampling strategy will be used for sampling./// "none"
    /// "mix"
    /// "gaussian"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_diffusion: Option<String>,
    /// Size of patch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_diffusion_size: Option<i64>,
    /// Stride of sliding patch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_diffusion_stride: Option<i64>,
    /// If specified, a patch-based sampling strategy will be used for VAE decoding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_vae: Option<bool>,
    /// Size of VAE patch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_vae_decoder_size: Option<i64>,
    /// Size of latent image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_vae_encoder_size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LightningModelsImageToImageInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://fal-cdn.batuhan-941.workers.dev/files/tiger/IExuP-WICqaIesLZAZPur.jpeg"
    pub image_url: String,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The Lightning model to use./// The Lightning model to use./// "Lykon/dreamshaper-xl-lightning"
    /// "SG161222/RealVisXL_V4.0_Lightning"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "an island near sea, with seagulls, moon shining over the sea, light house, boats int he background, fish flying over the sea"
    pub prompt: String,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// Scheduler / sampler to use for the image denoising process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SpanishOutput {
    /// The generated music/// The generated music/// {"url":"https://fal.media/files/monkey/5rBM3qVCED73Lxs5XLcwj_tmp4f2z_qrf.wav"}
    pub audio: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ValidationError {
    pub loc: Vec<serde_json::Value>,
    pub msg: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RGBColor {
    /// Blue color value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<i64>,
    /// Green color value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g: Option<i64>,
    /// Red color value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxSubjectInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// URL of image of the subject/// URL of image of the subject/// "https://storage.googleapis.com/falserverless/model_tests/ominicontrol/ominishirt.jpg"
    pub image_url: String,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "On the beach, a lady sits under a beach umbrella with 'Omini' written on it. She's wearing this shirt and has a big smile on her face, with her surfboard hehind her. The sun is setting in the background. The sky is a beautiful shade of orange and purple."
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
pub struct AudioInput {
    /// The URL of the audio file./// The URL of the audio file./// "https://storage.googleapis.com/falserverless/model_tests/f5-tts/en_1_ref.mp3"
    pub audio_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MusicOutput {
    /// The generated music/// The generated music/// {"url":"https://fal.media/files/elephant/N5UNLCwkC2y8v7a3LQLFE_output.mp3"}
    pub audio: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InpaintingLCMInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The rescale factor for the CFG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_rescale: Option<f64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
    pub image_url: String,
    /// The URL of the mask to use for inpainting./// The URL of the mask to use for inpainting./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
    pub mask_url: String,
    /// The name of the model to use./// The name of the model to use./// "stabilityai/stable-diffusion-xl-base-1.0"
    /// "runwayml/stable-diffusion-v1-5"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "a tiger sitting on a park bench"
    pub prompt: String,
    /// An id bound to a request, can be used with response to identify the request
    /// itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AmEngOutput {
    /// The generated music/// The generated music/// {"url":"https://fal.media/files/elephant/dXVMqWsBDG9yan3kaOT0Z_tmp0vvkha3s.wav"}
    pub audio: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProcessedOutput {
    /// The processed images
    pub images: Vec<Image>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Embedding {
    /// URL or the path to the embedding weights./// URL or the path to the embedding weights./// "https://civitai.com/api/download/models/135931"
    /// "https://filebin.net/3chfqasxpqu21y8n/my-custom-lora-v1.safetensors"
    pub path: String,
    /// The list of tokens to use for the embedding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<Option<String>>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OmniZeroOutput {
    /// The generated image./// The generated image./// {"content_type":"image/png","height":1024,"url":"https://storage.googleapis.com/falserverless/model_tests/omni_zero/result.png","width":1024}
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LoraWeight {
    /// If set to true, the embedding will be forced to be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// URL or the path to the LoRA weights. Or HF model name./// URL or the path to the LoRA weights. Or HF model name./// "https://civitai.com/api/download/models/135931"
    /// "https://filebin.net/3chfqasxpqu21y8n/my-custom-lora-v1.safetensors"
    pub path: String,
    /// The scale of the LoRA weight. This is used to scale the LoRA weight
    /// before merging it with the base model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TryOnRequest {
    /// Url to the garment image./// Url to the garment image./// "https://storage.googleapis.com/falserverless/model_tests/leffa/tshirt_image.jpg"
    pub garment_image_url: String,
    /// Url for the human image./// Url for the human image./// "https://storage.googleapis.com/falserverless/model_tests/leffa/person_image.jpg"
    pub human_image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PolygonOutputWithLabels {
    /// Processed image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Option<Image>>,
    /// Results from the model
    pub results: PolygonOutput,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OCRBoundingBox {
    /// List of quadrilateral boxes
    pub quad_boxes: Vec<OCRBoundingBoxSingle>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MLSDInput {
    /// Distance threshold for the MLSD detector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_threshold: Option<f64>,
    /// URL of the image to process/// URL of the image to process/// "https://storage.googleapis.com/falserverless/model_tests/image_preprocessors/cat.png"
    pub image_url: String,
    /// Score threshold for the MLSD detector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageInput {
    /// Input image url./// Input image url./// "https://storage.googleapis.com/falserverless/model_tests/workflow_utils/mask_input.png"
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TeeDOutput {
    /// Image with TeeD lines detected
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QueryInput {
    /// Image URL to be processed/// Image URL to be processed/// "https://llava-vl.github.io/static/images/monalisa.jpg"
    pub image_url: String,
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    /// Prompt for query task/// Prompt for query task/// "Describe this image in detail."
    pub prompt: String,
    /// Type of task to perform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct V3TTSOutput {
    /// The generated audio file./// The generated audio file./// {"content_type":"audio/mpeg","duration":2.3486666666666665,"file_name":"166db034-7421-4767-adad-ab7c36a99b75.mp3","file_size":57069,"url":"https://fal-api-audio-uploads.s3.amazonaws.com/166db034-7421-4767-adad-ab7c36a99b75.mp3"}
    pub audio: AudioFile,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BoxPrompt {
    /// The frame index to interact with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_index: Option<i64>,
    /// X Max Coordinate of the prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_max: Option<i64>,
    /// X Min Coordinate of the box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_min: Option<i64>,
    /// Y Max Coordinate of the prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_max: Option<i64>,
    /// Y Min Coordinate of the box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_min: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ControlNeXtInput {
    /// Number of frames to process in each batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_frames: Option<i64>,
    /// Condition scale for ControlNeXt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlnext_cond_scale: Option<f64>,
    /// Chunk size for decoding frames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decode_chunk_size: Option<i64>,
    /// Frames per second for the output video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<i64>,
    /// Guidance scale for the diffusion process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// Height of the output video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// URL of the reference image./// URL of the reference image./// "https://storage.googleapis.com/falserverless/model_tests/musepose/ref.png"
    pub image_url: String,
    /// Maximum number of frames to process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_frame_num: Option<i64>,
    /// Motion bucket ID for the pipeline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_bucket_id: Option<f64>,
    /// Number of inference steps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// Number of overlapping frames between batches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlap: Option<i64>,
    /// Stride for sampling frames from the input video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_stride: Option<i64>,
    /// URL of the input video./// URL of the input video./// "https://storage.googleapis.com/falserverless/model_tests/musepose/dance.mp4"
    pub video_url: String,
    /// Width of the output video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InpaintingSD15Input {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
    pub image_url: String,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The URL of the mask to use for inpainting./// The URL of the mask to use for inpainting./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
    pub mask_url: String,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "a tiger sitting on a park bench"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Era3DInput {
    /// Background removal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_removal: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfg: Option<f64>,
    /// Size of the image to crop to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop_size: Option<i64>,
    /// URL of the image to remove background from/// URL of the image to remove background from/// "https://storage.googleapis.com/falserverless/model_tests/era3d/DnvGjd9CCS-ESmLgTYgOn.png"
    pub image_url: String,
    /// Seed for random number generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Number of steps to run the model for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WanI2VRequest {
    /// Whether to enable prompt expansion./// Whether to enable prompt expansion./// true

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_prompt_expansion: Option<bool>,
    /// If set to true, the safety checker will be enabled./// If set to true, the safety checker will be enabled./// true

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// Frames per second of the generated video. Must be between 5 to 24.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frames_per_second: Option<i64>,
    /// URL of the input image./// URL of the input image./// "https://fal.media/files/elephant/8kkhB12hEZI2kkbU8pZPA_test.jpeg"
    pub image_url: String,
    /// Number of frames to generate. Must be between 81 to 100 (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_frames: Option<i64>,
    /// Number of inference steps for sampling. Higher values give better quality but take longer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The text prompt to guide video generation./// The text prompt to guide video generation./// "A stylish woman walks down a Tokyo street filled with warm glowing neon and animated city signage."
    pub prompt: String,
    /// Resolution of the generated video (480p or 720p).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// Random seed for reproducibility. If None, a random seed is chosen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageProcessingInput {
    /// Blue channel shift direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_direction: Option<String>,
    /// Blue channel shift amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_shift: Option<i64>,
    /// Blur radius
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur_radius: Option<i64>,
    /// Sigma for Gaussian blur
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur_sigma: Option<f64>,
    /// Type of blur to apply
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur_type: Option<String>,
    /// Brightness adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<f64>,
    /// CAS sharpening amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cas_amount: Option<f64>,
    /// Contrast adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrast: Option<f64>,
    /// Desaturation factor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desaturate_factor: Option<f64>,
    /// Desaturation method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desaturate_method: Option<String>,
    /// Dissolve blend factor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissolve_factor: Option<f64>,
    /// URL of second image for dissolve
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissolve_image_url: Option<String>,
    /// Dodge and burn intensity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dodge_burn_intensity: Option<f64>,
    /// Dodge and burn mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dodge_burn_mode: Option<String>,
    /// Enable blur effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_blur: Option<bool>,
    /// Enable chromatic aberration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_chromatic: Option<bool>,
    /// Enable color correction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_color_correction: Option<bool>,
    /// Enable desaturation effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_desaturate: Option<bool>,
    /// Enable dissolve effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_dissolve: Option<bool>,
    /// Enable dodge and burn effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_dodge_burn: Option<bool>,
    /// Enable glow effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_glow: Option<bool>,
    /// Enable film grain effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_grain: Option<bool>,
    /// Enable parabolize effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_parabolize: Option<bool>,
    /// Enable sharpen effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_sharpen: Option<bool>,
    /// Enable solarize effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_solarize: Option<bool>,
    /// Enable color tint effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_tint: Option<bool>,
    /// Enable vignette effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_vignette: Option<bool>,
    /// Gamma adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamma: Option<f64>,
    /// Glow intensity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glow_intensity: Option<f64>,
    /// Glow blur radius
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glow_radius: Option<i64>,
    /// Film grain intensity (when enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grain_intensity: Option<f64>,
    /// Film grain scale (when enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grain_scale: Option<f64>,
    /// Style of film grain to apply
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grain_style: Option<String>,
    /// Green channel shift direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green_direction: Option<String>,
    /// Green channel shift amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green_shift: Option<i64>,
    /// URL of image to process/// URL of image to process/// "https://storage.googleapis.com/falserverless/web-examples/post-process/postpro-input.jpg"
    pub image_url: String,
    /// Noise radius for smart sharpen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noise_radius: Option<i64>,
    /// Parabolize coefficient
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parabolize_coeff: Option<f64>,
    /// Edge preservation factor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_edges: Option<f64>,
    /// Red channel shift direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red_direction: Option<String>,
    /// Red channel shift amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red_shift: Option<i64>,
    /// Saturation adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturation: Option<f64>,
    /// Sharpen strength (for basic mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpen_alpha: Option<f64>,
    /// Type of sharpening to apply
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpen_mode: Option<String>,
    /// Sharpen radius (for basic mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpen_radius: Option<i64>,
    /// Smart sharpen blend ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_sharpen_ratio: Option<f64>,
    /// Smart sharpen strength
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_sharpen_strength: Option<f64>,
    /// Solarize threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solarize_threshold: Option<f64>,
    /// Color temperature adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Tint color mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint_mode: Option<String>,
    /// Tint strength
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint_strength: Option<f64>,
    /// Vertex X position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertex_x: Option<f64>,
    /// Vertex Y position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertex_y: Option<f64>,
    /// Vignette strength (when enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vignette_strength: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DreamshaperInpaintingInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
    pub image_url: String,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The URL of the mask to use for inpainting./// The URL of the mask to use for inpainting./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
    pub mask_url: String,
    /// The Dreamshaper model to use./// The Dreamshaper model to use./// "Lykon/dreamshaper-8"
    /// "Lykon/dreamshaper-xl-1-0"
    /// "Lykon/dreamshaper-xl-v2-turbo"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "a tiger sitting on a park bench"
    pub prompt: String,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LipSyncOutput {
    /// The generated video/// The generated video/// {"url":"https://v3.fal.media/files/rabbit/6gJV-z7RJsF0AxkZHkdgJ_output.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FlowEditInput {
    /// URL of image to be used for relighting/// URL of image to be used for relighting/// "https://storage.googleapis.com/falserverless/model_tests/FlowEdit/lighthouse.png"
    pub image_url: String,
    /// Average step count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_avg: Option<i64>,
    /// Control the strength of the edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_max: Option<i64>,
    /// Minimum step for improved style edits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_min: Option<i64>,
    /// Steps for which the model should run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// Random seed for reproducible generation. If set none, a random seed will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Prompt of the image to be used./// Prompt of the image to be used./// "The image features a tall white lighthouse standing prominently\n      on a hill, with a beautiful blue sky in the background. The lighthouse is illuminated\n      by a bright light, making it a prominent landmark in the scene."
    pub source_prompt: String,
    /// Guidance scale for the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_guidance_scale: Option<i64>,
    /// Guidance scale for target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tar_guidance_scale: Option<i64>,
    /// Prompt of the image to be made./// Prompt of the image to be made./// "The image features Big ben clock tower standing prominently\n      on a hill, with a beautiful blue sky in the background. The Big ben clock tower is illuminated\n      by a bright light, making it a prominent landmark in the scene."
    pub target_prompt: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VideoInput {
    /// Number of frames to sample from the video. If not provided, all frames are sampled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_frames_to_sample: Option<i64>,
    /// Prompt to be used for the chat completion/// Prompt to be used for the chat completion/// "Could you please give me a brief description of the video? Please respond with interleaved segmentation masks for the corresponding parts of the answer."
    pub prompt: String,
    /// The URL of the input video./// The URL of the input video./// "https://drive.google.com/uc?id=1iOFYbNITYwrebBBp9kaEGhBndFSRLz8k"
    pub video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrenchOutput {
    /// The generated music/// The generated music/// {"url":"https://fal.media/files/kangaroo/E_itKJKZKRNaO-QtU77k1_tmpe1qso5xp.wav"}
    pub audio: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageToImageInput {
    /// Skips part of the image generation process, leading to slightly different results.
    /// This means the image renders faster, too.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_skip: Option<i64>,
    /// If set to true, the controlnet will be applied to only the conditional predictions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlnet_guess_mode: Option<bool>,
    /// The control nets to use for the image generation. You can use any number of control nets
    /// and they will be applied to the image at the specified timesteps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlnets: Option<Vec<Option<ControlNet>>>,
    /// If set to true, the latents will be saved for debugging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_latents: Option<bool>,
    /// If set to true, the latents will be saved for debugging per pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_per_pass_latents: Option<bool>,
    /// The embeddings to use for the image generation. Only a single embedding is supported at the moment.
    /// The embeddings will be used to map the tokens in the prompt to the embedding weights.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The eta value to be used for the image generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eta: Option<f64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The URL of the IC Light model image to use for the image generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ic_light_image_url: Option<String>,
    /// The URL of the IC Light model background image to use for the image generation.
    /// Make sure to use a background compatible with the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ic_light_model_background_image_url: Option<String>,
    /// The URL of the IC Light model to use for the image generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ic_light_model_url: Option<String>,
    /// The path to the image encoder model to use for the image generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_encoder_path: Option<String>,
    /// The subfolder of the image encoder model to use for the image generation./// The subfolder of the image encoder model to use for the image generation.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_encoder_subfolder: Option<String>,
    /// The weight name of the image encoder model to use for the image generation./// The weight name of the image encoder model to use for the image generation./// "pytorch_model.bin"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_encoder_weight_name: Option<String>,
    /// The format of the generated image./// The format of the generated image./// "jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_format: Option<String>,
    /// URL of image to use for image to image/inpainting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// The IP adapter to use for the image generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_adapter: Option<Vec<Option<IPAdapter>>>,
    /// The LoRAs to use for the image generation. You can use any number of LoRAs
    /// and they will be merged together to generate the final image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// URL or HuggingFace ID of the base model to generate the image./// URL or HuggingFace ID of the base model to generate the image./// "stabilityai/stable-diffusion-xl-base-1.0"
    /// "runwayml/stable-diffusion-v1-5"
    /// "SG161222/Realistic_Vision_V2.0"
    pub model_name: String,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, painting, illustration, worst quality, low quality, normal quality"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The amount of noise to add to noise image for image. Only used if the image_url is provided. 1.0 is complete noise and 0 is no noise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noise_strength: Option<f64>,
    /// Number of images to generate in one request. Note that the higher the batch size,
    /// the longer it will take to generate the images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// Increasing the amount of steps tells Stable Diffusion that it should take more steps
    /// to generate your final result which can increase the amount of detail in your image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The type of prediction to use for the image generation.
    /// The `epsilon` is the default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction_type: Option<String>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "Photo of a european medieval 40 year old queen, silver hair, highly detailed face, detailed eyes, head shot, intricate crown, age spots, wrinkles"
    /// "Photo of a classic red mustang car parked in las vegas strip at night"
    pub prompt: String,
    /// If set to true, the prompt weighting syntax will be used.
    /// Additionally, this will lift the 77 token limit by averaging embeddings./// If set to true, the prompt weighting syntax will be used.
    /// Additionally, this will lift the 77 token limit by averaging embeddings./// true

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_weighting: Option<bool>,
    /// Whether to set the rescale_betas_snr_zero option or not for the sampler
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rescale_betas_snr_zero: Option<bool>,
    /// Scheduler / sampler to use for the image denoising process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Optionally override the sigmas to use for the denoising process. Only works with schedulers which support the `sigmas` argument in their `set_sigmas` method.
    /// Defaults to not overriding, in which case the scheduler automatically sets the sigmas based on the `num_inference_steps` parameter.
    /// If set to a custom sigma schedule, the `num_inference_steps` parameter will be ignored. Cannot be set if `timesteps` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sigmas: Option<Option<SigmasInput>>,
    /// The size of the tiles to be used for the image generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_height: Option<i64>,
    /// The stride of the tiles to be used for the image generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_stride_height: Option<i64>,
    /// The stride of the tiles to be used for the image generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_stride_width: Option<i64>,
    /// The size of the tiles to be used for the image generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_width: Option<i64>,
    /// Optionally override the timesteps to use for the denoising process. Only works with schedulers which support the `timesteps` argument in their `set_timesteps` method.
    /// Defaults to not overriding, in which case the scheduler automatically sets the timesteps based on the `num_inference_steps` parameter.
    /// If set to a custom timestep schedule, the `num_inference_steps` parameter will be ignored. Cannot be set if `sigmas` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timesteps: Option<Option<TimestepsInput>>,
    /// URL or HuggingFace ID of the custom U-Net model to use for the image generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unet_name: Option<String>,
    /// The variant of the model to use for huggingface models, e.g. 'fp16'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProRedux {
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The image URL to generate an image from. Needs to match the dimensions of the mask./// The image URL to generate an image from. Needs to match the dimensions of the mask./// "https://fal.media/files/kangaroo/acQvq-Kmo2lajkgvcEHdv.png"
    pub image_url: String,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to generate an image from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_tolerance: Option<String>,
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
pub struct RetoucherInput {
    /// The URL of the image to be retouched./// The URL of the image to be retouched./// "https://storage.googleapis.com/falserverless/model_tests/retoucher/Dalton-Meereskosmetik-Magazin-Pickelguide-Model_1.resized.jpg"
    pub image_url: String,
    /// Seed for reproducibility. Different seeds will make slightly different results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PhotoMakerInput {
    /// The base pipeline to use for generating the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_pipeline: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The URL of the image archive containing the images you want to use./// The URL of the image archive containing the images you want to use./// "https://storage.googleapis.com/falserverless/model_tests/photomaker/elon.zip"
    pub image_archive_url: String,
    /// How much noise to add to the latent image. O for no noise, 1 for maximum noise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_image_strength: Option<f64>,
    /// Optional initial image for img2img
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_image_url: Option<String>,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "nsfw, lowres, bad anatomy, bad hands, text, error, missing fingers, extra digit, fewer digits, cropped, worst quality, low quality, normal quality, jpeg artifacts, signature, watermark, username, blurry"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Number of images to generate in one request. Note that the higher the batch size,
    /// the longer it will take to generate the images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// Increasing the amount of steps tells Stable Diffusion that it should take more steps
    /// to generate your final result which can increase the amount of detail in your image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "instagram photo, portrait photo of a man img, colorful, perfect face, natural skin, hard shadows, film grain"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// 42

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_strength: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RetoucherOutput {
    /// The generated image file info./// The generated image file info./// {"url":"https://storage.googleapis.com/falserverless/model_tests/retoucher/retoucher_example_output.png"}
    pub image: Image,
    /// The seed used for the generation.
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MiniCPMV26ImageInput {
    /// List of image URLs to be used for the image description/// List of image URLs to be used for the image description/// ["https://llava-vl.github.io/static/images/monalisa.jpg"]
    pub image_urls: Vec<String>,
    /// Prompt to be used for the image description/// Prompt to be used for the image description/// "Who is she? Who drew this?"
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct T2IOutput {
    /// The generated image
    pub images: Vec<File>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProUltraTextToImageInputRedux {
    /// The aspect ratio of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatioProperty>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The strength of the image prompt, between 0 and 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_prompt_strength: Option<f64>,
    /// The image URL to generate an image from. Needs to match the dimensions of the mask./// The image URL to generate an image from. Needs to match the dimensions of the mask./// "https://fal.media/files/kangaroo/acQvq-Kmo2lajkgvcEHdv.png"
    pub image_url: String,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to generate an image from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Generate less processed, more natural-looking images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<bool>,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_tolerance: Option<String>,
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
pub struct MandarinOutput {
    /// The generated music/// The generated music/// {"url":"https://fal.media/files/rabbit/8UiqobkQXPrYDRHl4l5oU_tmptz6jo3ex.wav"}
    pub audio: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PhotoMakerOutput {
    pub images: Vec<Image>,
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HunyuanV2VRequest {
    /// The aspect ratio of the video to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// If set to true, the safety checker will be enabled./// If set to true, the safety checker will be enabled./// true

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The number of frames to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_frames: Option<String>,
    /// The number of inference steps to run. Lower gets faster results, higher gets better results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// By default, generations are done with 35 steps. Pro mode does 55 steps which results in higher quality videos but will take more time and cost 2x more billing units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pro_mode: Option<bool>,
    /// The prompt to generate the video from./// The prompt to generate the video from./// "A stylish woman walks down a Tokyo street filled with warm glowing neon and animated city signage. She wears a dark blue leather jacket, a long pink dress, and bright yellow boots, and carries a black purse."
    pub prompt: String,
    /// The resolution of the video to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// The seed to use for generating the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Strength for Video-to-Video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// URL of the video input./// URL of the video input./// "https://storage.googleapis.com/falserverless/hunyuan_video/hunyuan_v2v_input.mp4"
    pub video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PoseTransferInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your input when generating the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// Url to the garment image./// Url to the garment image./// "https://storage.googleapis.com/falserverless/model_tests/leffa/pose_image.jpg"
    pub person_image_url: String,
    /// Url for the human image./// Url for the human image./// "https://storage.googleapis.com/falserverless/model_tests/leffa/person_image.jpg"
    pub pose_image_url: String,
    /// The same seed and the same input given to the same version of the model
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
pub struct AudioFile {
    /// The mime type of the file./// The mime type of the file./// "image/png"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The duration of the audio file in seconds.
    pub duration: f64,
    /// File data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>,
    /// The name of the file. It will be auto-generated if not provided./// The name of the file. It will be auto-generated if not provided./// "z9RV14K95DvU.png"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// The size of the file in bytes./// The size of the file in bytes./// 4404019

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// The URL where the file can be downloaded from.
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateVoiceInput {
    /// Voice name (required, max 255 characters)./// Voice name (required, max 255 characters)./// "my voice"
    pub name: String,
    /// A list of audio URLs used for cloning (must be between 1 and 5 URLs)./// A list of audio URLs used for cloning (must be between 1 and 5 URLs)./// [{"audio_url":"https://storage.googleapis.com/falserverless/model_tests/f5-tts/en_1_ref.mp3"}]
    pub samples: Vec<AudioInput>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ControlNetUnionInput {
    /// The scale of the control net weight. This is used to scale the control net weight
    /// before merging it with the base model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditioning_scale: Option<f64>,
    /// URL of the image to be used as the control image.
    pub control_image_url: String,
    /// Control Mode for Flux Controlnet Union. Supported values are:
    /// - canny: Uses the edges for guided generation.
    /// - tile: Uses the tiles for guided generation.
    /// - depth: Utilizes a grayscale depth map for guided generation.
    /// - blur: Adds a blur to the image.
    /// - pose: Uses the pose of the image for guided generation.
    /// - gray: Converts the image to grayscale.
    /// - low-quality: Converts the image to a low-quality image.
    pub control_mode: String,
    /// The percentage of the image to end applying the controlnet in terms of the total timesteps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_percentage: Option<f64>,
    /// URL of the mask for the control image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_image_url: Option<String>,
    /// Threshold for mask.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_threshold: Option<f64>,
    /// The percentage of the image to start applying the controlnet in terms of the total timesteps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_percentage: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageToVideoDirectorRequest {
    /// URL of the image to use as the first frame/// URL of the image to use as the first frame/// "https://fal.media/files/elephant/8kkhB12hEZI2kkbU8pZPA_test.jpeg"
    pub image_url: String,
    /// Text prompt for video generation. Camera movement instructions can be added using square brackets (e.g. [Pan left] or [Zoom in]). You can use up to 3 combined movements per prompt. Supported movements: Truck left/right, Pan left/right, Push in/Pull out, Pedestal up/down, Tilt up/down, Zoom in/out, Shake, Tracking shot, Static shot. For example: [Truck left, Pan right, Zoom in]. For a more detailed guide, refer https://sixth-switch-2ac.notion.site/T2V-01-Director-Model-Tutorial-with-camera-movement-1886c20a98eb80f395b8e05291ad8645/// Text prompt for video generation. Camera movement instructions can be added using square brackets (e.g. [Pan left] or [Zoom in]). You can use up to 3 combined movements per prompt. Supported movements: Truck left/right, Pan left/right, Push in/Pull out, Pedestal up/down, Tilt up/down, Zoom in/out, Shake, Tracking shot, Static shot. For example: [Truck left, Pan right, Zoom in]. For a more detailed guide, refer https://sixth-switch-2ac.notion.site/T2V-01-Director-Model-Tutorial-with-camera-movement-1886c20a98eb80f395b8e05291ad8645/// "[Push in, Follow]A stylish woman walks down a Tokyo street filled with warm glowing neon and animated city signage. She wears a black leather jacket, a long red dress, and black boots, and carries a black purse.[Pan left] The street opens into a small plaza where street vendors sell steaming food under colorful awnings."
    pub prompt: String,
    /// Whether to use the model's prompt optimizer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_optimizer: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToImageRequest {
    /// The aspect ratio of the generated video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpscaleOutput {
    /// The upscaled image.
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FooocusInpaintInput {
    /// The size of the generated image. You can choose between some presets or
    /// custom height and width that **must be multiples of 8**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// If set to false, the safety checker will be disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_prompt_1: Option<ImagePrompt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_prompt_2: Option<ImagePrompt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_prompt_3: Option<ImagePrompt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_prompt_4: Option<ImagePrompt>,
    /// Describe what you want to inpaint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inpaint_additional_prompt: Option<String>,
    /// If set to true, the initial preprocessing will be disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inpaint_disable_initial_latent: Option<bool>,
    /// Version of Fooocus inpaint model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inpaint_engine: Option<String>,
    /// Positive value will make white area in the mask larger, negative value will
    /// make white area smaller. (default is 0, always process before any mask
    /// invert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inpaint_erode_or_dilate: Option<f64>,
    /// The image to use as a reference for inpainting./// The image to use as a reference for inpainting./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
    pub inpaint_image_url: String,
    /// The mode to use for inpainting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inpaint_mode: Option<String>,
    /// The area to inpaint. Value 0 is same as "Only Masked" in A1111. Value 1 is
    /// same as "Whole Image" in A1111. Only used in inpaint, not used in outpaint.
    /// (Outpaint always use 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inpaint_respective_field: Option<f64>,
    /// Same as the denoising strength in A1111 inpaint. Only used in inpaint, not
    /// used in outpaint. (Outpaint always use 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inpaint_strength: Option<f64>,
    /// If set to true, the mask will be inverted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert_mask: Option<bool>,
    /// The LoRAs to use for the image generation. You can use up to 5 LoRAs
    /// and they will be merged together to generate the final image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The image to use as a mask for the generated image./// The image to use as a mask for the generated image./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_image_url: Option<String>,
    /// Mixing Image Prompt and Inpaint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixing_image_prompt_and_inpaint: Option<bool>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "(worst quality, low quality, normal quality, lowres, low details, oversaturated, undersaturated, overexposed, underexposed, grayscale, bw, bad photo, bad photography, bad art:1.4), (watermark, signature, text font, username, error, logo, words, letters, digits, autograph, trademark, name:1.2), (blur, blurry, grainy), morbid, ugly, asymmetrical, mutated malformed, mutilated, poorly lit, bad shadow, draft, cropped, out of frame, cut off, censored, jpeg artifacts, out of focus, glitch, duplicate, (airbrushed, cartoon, anime, semi-realistic, cgi, render, blender, digital art, manga, amateur:1.3), (3D ,3D Game, 3D Game Scene, 3D Character:1.1), (bad hands, bad anatomy, bad body, bad face, bad teeth, bad arms, bad legs, deformities:1.3)"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Number of images to generate in one request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The directions to outpaint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpaint_selections: Option<Vec<Option<String>>>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// If set to true, the advanced inpaint options ('inpaint_disable_initial_latent',
    /// 'inpaint_engine', 'inpaint_strength', 'inpaint_respective_field',
    /// 'inpaint_erode_or_dilate') will be overridden.
    /// Otherwise, the default values will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_inpaint_options: Option<bool>,
    /// You can choose Speed or Quality
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<String>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "a cat on a bench, realistic, highly detailed, 8k"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Refiner (SDXL or SD 1.5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refiner_model: Option<String>,
    /// Use 0.4 for SD1.5 realistic models; 0.667 for SD1.5 anime models
    /// 0.8 for XL-refiners; or any value for switching two SDXL models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refiner_switch: Option<f64>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// 176400

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The sharpness of the generated image. Use it to control how sharp the generated
    /// image should be. Higher value means image and texture are sharper.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpness: Option<f64>,
    /// The style to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<Vec<Option<String>>>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MarigoldDepthMapOutput {
    /// The depth map.
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InpaintOutput {
    /// The generated image files info.
    pub image: Image,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProOutpaintInput {
    /// Pixels to expand at the bottom
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_bottom: Option<i64>,
    /// Pixels to expand on the left
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_left: Option<i64>,
    /// Pixels to expand on the right
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_right: Option<i64>,
    /// Pixels to expand at the top
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_top: Option<i64>,
    /// The image URL to expand using outpainting/// The image URL to expand using outpainting/// "https://storage.googleapis.com/falserverless/flux-lora/example-images/knight.jpeg"
    pub image_url: String,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_tolerance: Option<String>,
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
pub struct InpaintingHyperInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
    pub image_url: String,
    /// The URL of the mask to use for inpainting./// The URL of the mask to use for inpainting./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
    pub mask_url: String,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<String>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "a tiger sitting on a park bench"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Ben2InputImage {
    /// URL of image to be used for background removal/// URL of image to be used for background removal/// "https://storage.googleapis.com/falserverless/gallery/Ben2/arduino-uno-board-electronics-hand-600nw-1869855883.webp"
    pub image_url: String,
    /// Random seed for reproducible generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MuseTalkOutput {
    /// The generated video file.
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageChatOutput {
    /// Dictionary of label: mask image/// Dictionary of label: mask image/// [{"content_type":"image/png","file_name":"019c3c1e3c50446e9996f709d36debb4.png","file_size":15724,"height":1200,"url":"https://v3.fal.media/files/monkey/6ITmhHQJ-69s-UxajrY5T_019c3c1e3c50446e9996f709d36debb4.png","width":1800},{"content_type":"image/png","file_name":"0a1522ca410942c7ad6c73efa15b3549.png","file_size":14905,"height":1200,"url":"https://v3.fal.media/files/monkey/IljtMxahoo9-7SUpx0fth_0a1522ca410942c7ad6c73efa15b3549.png","width":1800}]
    pub masks: Vec<Image>,
    /// Generated output/// Generated output/// "<p>  A white pickup truck  </p>   [SEG]  is parked on the side of  <p>  the red building  </p>   [SEG] , creating a unique and eye-catching contrast.<|im_end|>"
    pub output: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AnimateDiffV2VInput {
    /// The first N number of seconds of video to animate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_n_seconds: Option<i64>,
    /// Number of frames per second to extract from the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<i64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The motions to apply to the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motions: Option<Vec<Option<String>>>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "closeup of tony stark, robert downey jr, fireworks, high quality, ultra HD"
    /// "panda playing a guitar, on a boat, in the ocean, high quality, high quality, ultra HD, realistic"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The strength of the input video in the final output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// URL of the video./// URL of the video./// "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-vid2vid-input-2.gif"
    /// "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-vid2vid-input-1.gif"
    pub video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RealisticVisionImageToImageInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://fal-cdn.batuhan-941.workers.dev/files/tiger/IExuP-WICqaIesLZAZPur.jpeg"
    pub image_url: String,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The Realistic Vision model to use./// The Realistic Vision model to use./// "SG161222/Realistic_Vision_V6.0_B1_noVAE"
    /// "SG161222/RealVisXL_V4.0"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "an island near sea, with seagulls, moon shining over the sea, light house, boats int he background, fish flying over the sea"
    pub prompt: String,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageConditioningInput {
    /// URL of image to use as conditioning
    pub image_url: String,
    /// Frame number of the image from which the conditioning starts. Must be a multiple of 8.
    pub start_frame_num: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LivePortraitVideoInput {
    /// Whether to prioritize source or driving audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_priority: Option<String>,
    /// URL of the video to drive the lip syncing./// URL of the video to drive the lip syncing./// "https://storage.googleapis.com/falserverless/model_tests/live-portrait/liveportrait-example.mp4"
    pub driving_video_url: String,
    /// Whether to filter out NSFW content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// URL of the video to drive the lip syncing./// URL of the video to drive the lip syncing./// "https://storage.googleapis.com/falserverless/videos/s13.mp4"
    pub source_video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VideoToVideoInput {
    /// The target FPS of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_fps: Option<i64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related video to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The LoRAs to use for the image generation. We currently support one lora./// The LoRAs to use for the image generation. We currently support one lora.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The negative prompt to generate video from/// The negative prompt to generate video from/// "Distorted, discontinuous, Ugly, blurry, low resolution, motionless, static, disfigured, disconnected limbs, Ugly faces, incomplete arms"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to generate the video from./// The prompt to generate the video from./// "An astronaut stands triumphantly at the peak of a towering mountain. Panorama of rugged peaks and valleys. Very futuristic vibe and animated aesthetic. Highlights of purple and golden colors in the scene. The sky is looks like an animated/cartoonish dream of galaxies, nebulae, stars, planets, moons, but the remainder of the scene is mostly realistic. "
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same video every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The strength to use for Video to Video.  1.0 completely remakes the video while 0.0 preserves the original.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// Use RIFE for video interpolation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_rife: Option<bool>,
    /// The size of the generated video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_size: Option<VideoSizeProperty>,
    /// The video to generate the video from./// The video to generate the video from./// "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/hiker.mp4"
    pub video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FaceToStickerOutput {
    /// Whether the generated images contain NSFW concepts.
    /// The key is the image type and the value is a boolean./// Whether the generated images contain NSFW concepts.
    /// The key is the image type and the value is a boolean./// {"sticker_image":false,"sticker_image_background_removed":false}
    pub has_nsfw_concepts: HasNsfwConcepts,
    /// The generated images./// The generated images./// [{"content_type":"image/PNG","file_name":"cd8bab71b946470099d5fa20c7eed440.png","file_size":560358,"height":1024,"url":"https://storage.googleapis.com/falserverless/model_tests/face_to_sticker/elon_output_1.png","width":1024},{"content_type":"image/PNG","file_name":"181ae8fa12534c6f9285a991b415d9a7.png","file_size":452906,"height":1024,"url":"https://storage.googleapis.com/falserverless/model_tests/face_to_sticker/elon_output_2.png","width":1024}]
    pub images: Vec<Image>,
    /// Seed used during the inference./// Seed used during the inference./// 3625437076
    pub seed: i64,
    /// The generated face sticker image./// The generated face sticker image./// {"content_type":"image/PNG","file_name":"cd8bab71b946470099d5fa20c7eed440.png","file_size":560358,"height":1024,"url":"https://storage.googleapis.com/falserverless/model_tests/face_to_sticker/elon_output_1.png","width":1024}
    pub sticker_image: Image,
    /// The generated face sticker image with the background removed./// The generated face sticker image with the background removed./// {"content_type":"image/PNG","file_name":"181ae8fa12534c6f9285a991b415d9a7.png","file_size":452906,"height":1024,"url":"https://storage.googleapis.com/falserverless/model_tests/face_to_sticker/elon_output_2.png","width":1024}
    pub sticker_image_background_removed: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SkyreelsI2VResponse {
    /// The seed used for generation/// The seed used for generation/// 42
    pub seed: i64,
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct KlingV1I2VOutput {
    /// The generated video/// The generated video/// {"url":"https://v3.fal.media/files/kangaroo/VUmAU3JvzS7mxwdgSU9zj_output.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageExpansionInput {
    /// The desired size of the final image, after the expansion. should have an area of less than 5000x5000 pixels./// The desired size of the final image, after the expansion. should have an area of less than 5000x5000 pixels./// [1200,674]
    pub canvas_size: Vec<i64>,
    /// The URL of the input image./// The URL of the input image./// "https://storage.googleapis.com/falserverless/model_tests/orange.png"
    pub image_url: String,
    /// The negative prompt you would like to use to generate images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Number of Images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The desired location of the original image, inside the full canvas. Provide the location of the upper left corner of the original image. The location can also be outside the canvas (the original image will be cropped)./// The desired location of the original image, inside the full canvas. Provide the location of the upper left corner of the original image. The location can also be outside the canvas (the original image will be cropped)./// [301,-66]
    pub original_image_location: Vec<i64>,
    /// The desired size of the original image, inside the full canvas. Ensure that the ratio of input image foreground or main subject to the canvas area is greater than 15% to achieve optimal results./// The desired size of the original image, inside the full canvas. Ensure that the ratio of input image foreground or main subject to the canvas area is greater than 15% to achieve optimal results./// [610,855]
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
pub struct FastSVDImageInput {
    /// The conditoning augmentation determines the amount of noise that will be
    /// added to the conditioning frame. The higher the number, the more noise
    /// there will be, and the less the video will look like the initial image.
    /// Increase it for more motion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cond_aug: Option<f64>,
    /// The FPS of the generated video. The higher the number, the faster the video will
    /// play. Total video length is 25 frames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<i64>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://storage.googleapis.com/falserverless/model_tests/svd/rocket.png"
    /// "https://storage.googleapis.com/falserverless/model_tests/svd/mustang.png"
    /// "https://storage.googleapis.com/falserverless/model_tests/svd/ship.png"
    /// "https://storage.googleapis.com/falserverless/model_tests/svd/rocket2.png"
    pub image_url: String,
    /// The motion bucket id determines the motion of the generated video. The
    /// higher the number, the more motion there will be.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_bucket_id: Option<i64>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The number of steps to run the model for. The higher the number the better
    /// the quality and longer it will take to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InsightfaceOutput {
    /// Bounding box of the face./// Bounding box of the face./// [0,0,100,100]
    pub bbox: Vec<f64>,
    /// Confidence score of the detection./// Confidence score of the detection./// 0.9
    pub det_score: f64,
    /// Embedding of the face./// Embedding of the face./// ""
    pub embedding_file: File,
    /// faces detected sorted by size
    pub faces: Vec<FaceDetection>,
    /// Keypoints of the face./// Keypoints of the face.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kps: Option<Vec<Option<Vec<Option<f64>>>>>,
    /// Keypoints of the face on the image./// Keypoints of the face on the image.
    pub kps_image: Image,
    /// Either M or F if available./// Either M or F if available./// "M"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DWPoseOutput {
    /// The predicted pose image
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ResizeToPixelsInput {
    /// If set, the output dimensions will be divisible by this value./// If set, the output dimensions will be divisible by this value./// 32

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_divisibility: Option<i64>,
    /// Input image url./// Input image url./// "https://storage.googleapis.com/falserverless/model_tests/retoucher/GGsAolHXsAA58vn.jpeg"
    pub image_url: String,
    /// Maximum number of pixels in the output image./// Maximum number of pixels in the output image./// 1000000

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pixels: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TTSInput {
    /// The text to be converted to speech./// The text to be converted to speech./// "I don't really care what you call me. I've been a silent spectator, watching species evolve, empires rise and fall. But always remember, I am mighty and enduring. Respect me and I'll nurture you; ignore me and you shall face the consequences."
    pub gen_text: String,
    /// The name of the model to be used for TTS.
    pub model_type: String,
    /// The URL of the reference audio file./// The URL of the reference audio file./// "https://github.com/SWivid/F5-TTS/raw/21900ba97d5020a5a70bcc9a0575dc7dec5021cb/tests/ref_audio/test_en_1_ref_short.wav"
    pub ref_audio_url: String,
    /// The reference text to be used for TTS. If not provided, an ASR (Automatic Speech Recognition) model will be used to generate the reference text./// The reference text to be used for TTS. If not provided, an ASR (Automatic Speech Recognition) model will be used to generate the reference text./// "Some call me nature, others call me mother nature."

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_text: Option<String>,
    /// Whether to remove the silence from the audio file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_silence: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AmEnglishRequest {
    pub prompt: String,
    /// Voice ID for the desired voice./// Voice ID for the desired voice./// "af_heart"
    pub voice: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PublicInput {
    /// The format of the archive. If not specified, the format will be inferred from the URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_archive_format: Option<String>,
    /// URL to zip archive with images of a consistent style. Try to use at least 10 images, although more is better.
    ///
    /// In addition to images the archive can contain text files with captions. Each text file should have the same name as the image file it corresponds to.
    ///
    /// The captions can include a special string `[trigger]`. If a trigger_word is specified, it will replace `[trigger]` in the captions.
    pub images_data_url: String,
    /// Learning rate to use for training.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learning_rate: Option<f64>,
    /// If True, multiresolution training will be used./// If True, multiresolution training will be used./// true

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiresolution_training: Option<bool>,
    /// URL to a checkpoint to resume training from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_from_checkpoint: Option<String>,
    /// Number of steps to train the LoRA on./// Number of steps to train the LoRA on./// 2500

    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<i64>,
    /// If True, the subject will be cropped from the image./// If True, the subject will be cropped from the image./// true

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_crop: Option<bool>,
    /// Trigger phrase to be used in the captions. If None, a trigger word will not be used.
    /// If no captions are provide the trigger_work will be used instead of captions. If captions are provided, the trigger word will replace the `[trigger]` string in the captions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_phrase: Option<String>,
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
pub struct FluxProTextToImageFinetunedInput {
    /// References your specific model
    pub finetune_id: String,
    /// Controls finetune influence.
    /// Increase this value if your target concept isn't showing up strongly enough.
    /// The optimal setting depends on your finetune and prompt
    pub finetune_strength: f64,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_tolerance: Option<String>,
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
pub struct HunyuanT2VResponse {
    /// The seed used for generating the video.
    pub seed: i64,
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FooocusLegacyInput {
    /// The size of the generated image. You can choose between some presets or
    /// custom height and width that **must be multiples of 8**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// The stop at value of the control image. Use it to control how much the generated image
    /// should look like the control image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_image_stop_at: Option<f64>,
    /// The image to use as a reference for the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_image_url: Option<String>,
    /// The strength of the control image. Use it to control how much the generated image
    /// should look like the control image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_image_weight: Option<f64>,
    /// The type of image control/// The type of image control/// "ImagePrompt"
    /// "PyraCanny"
    /// "CPDS"
    /// "FaceSwap"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_type: Option<String>,
    /// If set to false, the safety checker will be disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The image to use as a reference for inpainting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inpaint_image_url: Option<String>,
    /// The LoRAs to use for the image generation. You can use up to 5 LoRAs
    /// and they will be merged together to generate the final image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The image to use as a mask for the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixing_image_prompt_and_inpaint: Option<bool>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "(worst quality, low quality, normal quality, lowres, low details, oversaturated, undersaturated, overexposed, underexposed, grayscale, bw, bad photo, bad photography, bad art:1.4), (watermark, signature, text font, username, error, logo, words, letters, digits, autograph, trademark, name:1.2), (blur, blurry, grainy), morbid, ugly, asymmetrical, mutated malformed, mutilated, poorly lit, bad shadow, draft, cropped, out of frame, cut off, censored, jpeg artifacts, out of focus, glitch, duplicate, (airbrushed, cartoon, anime, semi-realistic, cgi, render, blender, digital art, manga, amateur:1.3), (3D ,3D Game, 3D Game Scene, 3D Character:1.1), (bad hands, bad anatomy, bad body, bad face, bad teeth, bad arms, bad legs, deformities:1.3)"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Number of images to generate in one request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// You can choose Speed or Quality
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<String>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "an astronaut in the jungle, cold color palette with butterflies in the background, highly detailed, 8k"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Refiner (SDXL or SD 1.5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refiner_model: Option<String>,
    /// Use 0.4 for SD1.5 realistic models; 0.667 for SD1.5 anime models
    /// 0.8 for XL-refiners; or any value for switching two SDXL models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refiner_switch: Option<f64>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// 176400

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The sharpness of the generated image. Use it to control how sharp the generated
    /// image should be. Higher value means image and texture are sharper.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpness: Option<f64>,
    /// The style to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<Vec<Option<String>>>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RealisticVisionInpaintingInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
    pub image_url: String,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The URL of the mask to use for inpainting./// The URL of the mask to use for inpainting./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
    pub mask_url: String,
    /// The Realistic Vision model to use./// The Realistic Vision model to use./// "SG161222/Realistic_Vision_V6.0_B1_noVAE"
    /// "SG161222/RealVisXL_V4.0"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "a tiger sitting on a park bench"
    pub prompt: String,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NafnetOutput {
    /// The generated image file info./// The generated image file info./// {"content_type":"image/png","file_name":"36d3ca4791a647678b2ff01a35c87f5a.png","file_size":423052,"height":512,"url":"https://storage.googleapis.com/falserverless/nafnet/2cbfd460e25344a69fa8077808fb484f.png","width":512}
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WhisperInput {
    /// URL of the audio file to transcribe. Supported formats: mp3, mp4, mpeg, mpga, m4a, wav or webm./// URL of the audio file to transcribe. Supported formats: mp3, mp4, mpeg, mpga, m4a, wav or webm./// "https://ihlhivqvotguuqycfcvj.supabase.co/storage/v1/object/public/public-text-to-speech/scratch-testing/earth-history-19mins.mp3"
    pub audio_url: String,
    /// Level of the chunks to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_level: Option<String>,
    /// Language of the audio file.
    /// If translate is selected as the task, the audio will be translated to
    /// English, regardless of the language selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Task to perform on the audio file. Either transcribe or translate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
    /// Version of the model to use. All of the models are the Whisper large variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageWithUserCoordinatesInput {
    /// The URL of the image to be processed./// The URL of the image to be processed./// "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/tasks/car.jpg"
    /// "http://ecx.images-amazon.com/images/I/51UUzBDAMsL.jpg"
    pub image_url: String,
    /// The user input coordinates/// The user input coordinates/// {"x1":100,"x2":200,"y1":100,"y2":200}
    pub region: Region,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct I2VOutput {
    /// The generated video/// The generated video/// {"content_type":"video/mp4","file_name":"output.mp4","file_size":4060052,"url":"https://fal.media/files/tiger/8V9H8RLyFiWjmJDOxGbcG_output.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageToImageLightningInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://fal-cdn.batuhan-941.workers.dev/files/tiger/IExuP-WICqaIesLZAZPur.jpeg"
    pub image_url: String,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<String>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "an island near sea, with seagulls, moon shining over the sea, light house, boats int he background, fish flying over the sea"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SAM2AutomaticSegmentationOutput {
    /// Combined segmentation mask.
    pub combined_mask: Image,
    /// Individual segmentation masks.
    pub individual_masks: Vec<Image>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Resolution {
    /// Display aspect ratio (e.g., '16:9')
    pub aspect_ratio: String,
    /// Height of the video in pixels
    pub height: i64,
    /// Width of the video in pixels
    pub width: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct T2VOutput {
    /// The generated video/// The generated video/// {"url":"https://v2.fal.media/files/fb33a862b94d4d7195e610e4cbc5d392_output.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InpaintingControlNetInput {
    /// The URL of the control image./// The URL of the control image./// "https://avatars.githubusercontent.com/u/74778219"
    pub control_image_url: String,
    /// The scale of the controlnet conditioning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlnet_conditioning_scale: Option<f64>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image. Leave it none to automatically infer from the control image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
    pub image_url: String,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The URL of the mask to use for inpainting./// The URL of the mask to use for inpainting./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
    pub mask_url: String,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "Ice fortress, aurora skies, polar wildlife, twilight"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TripoSRInput {
    /// Whether to remove the background from the input image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_remove_background: Option<bool>,
    /// Ratio of the foreground image to the original image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground_ratio: Option<f64>,
    /// Path for the image file to be processed./// Path for the image file to be processed./// "https://raw.githubusercontent.com/VAST-AI-Research/TripoSR/ea034e12a428fa848684a3f9f267b2042d298ca6/examples/hamburger.png"
    /// "https://raw.githubusercontent.com/VAST-AI-Research/TripoSR/ea034e12a428fa848684a3f9f267b2042d298ca6/examples/poly_fox.png"
    /// "https://raw.githubusercontent.com/VAST-AI-Research/TripoSR/ea034e12a428fa848684a3f9f267b2042d298ca6/examples/robot.png"
    /// "https://raw.githubusercontent.com/VAST-AI-Research/TripoSR/ea034e12a428fa848684a3f9f267b2042d298ca6/examples/teapot.png"
    /// "https://raw.githubusercontent.com/VAST-AI-Research/TripoSR/ea034e12a428fa848684a3f9f267b2042d298ca6/examples/tiger_girl.png"
    /// "https://raw.githubusercontent.com/VAST-AI-Research/TripoSR/ea034e12a428fa848684a3f9f267b2042d298ca6/examples/horse.png"
    /// "https://raw.githubusercontent.com/VAST-AI-Research/TripoSR/ea034e12a428fa848684a3f9f267b2042d298ca6/examples/flamingo.png"
    /// "https://raw.githubusercontent.com/VAST-AI-Research/TripoSR/ea034e12a428fa848684a3f9f267b2042d298ca6/examples/unicorn.png"
    /// "https://raw.githubusercontent.com/VAST-AI-Research/TripoSR/ea034e12a428fa848684a3f9f267b2042d298ca6/examples/chair.png"
    /// "https://raw.githubusercontent.com/VAST-AI-Research/TripoSR/ea034e12a428fa848684a3f9f267b2042d298ca6/examples/iso_house.png"
    /// "https://raw.githubusercontent.com/VAST-AI-Research/TripoSR/ea034e12a428fa848684a3f9f267b2042d298ca6/examples/marble.png"
    /// "https://raw.githubusercontent.com/VAST-AI-Research/TripoSR/ea034e12a428fa848684a3f9f267b2042d298ca6/examples/police_woman.png"
    /// "https://raw.githubusercontent.com/VAST-AI-Research/TripoSR/ea034e12a428fa848684a3f9f267b2042d298ca6/examples/captured_p.png"
    pub image_url: String,
    /// Resolution of the marching cubes. Above 512 is not recommended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mc_resolution: Option<i64>,
    /// Output format for the 3D model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SAM2VideoOutput {
    /// The segmented video.
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DevTextToImageInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
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
pub struct Ben2InputVideo {
    /// Random seed for reproducible generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// URL of video to be used for background removal./// URL of video to be used for background removal./// "https://storage.googleapis.com/falserverless/gallery/Ben2/100063-video-2160.mp4"
    pub video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StableCascadeInput {
    /// If set to false, the safety checker will be disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// Number of steps to run the first stage for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_stage_steps: Option<i64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "ugly, deformed"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "An image of a shiba inu, donning a spacesuit and helmet"
    pub prompt: String,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_stage_guidance_scale: Option<f64>,
    /// Number of steps to run the second stage for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_stage_steps: Option<i64>,
    /// The same seed and the same prompt given to the same version of Stable Cascade
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// If set to true, the image will be returned as base64 encoded string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AMTFrameInterpolationInput {
    /// Frames to interpolate/// Frames to interpolate/// [{"url":"https://storage.googleapis.com/falserverless/model_tests/amt-interpolation/start.png"},{"url":"https://storage.googleapis.com/falserverless/model_tests/amt-interpolation/end.png"}]
    pub frames: Vec<Frame>,
    /// Output frames per second
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_fps: Option<i64>,
    /// Number of recursive interpolation passes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive_interpolation_passes: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MaskInput {
    /// Input image url./// Input image url./// "https://storage.googleapis.com/falserverless/model_tests/workflow_utils/mask_input.png"
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RGBAToRGBImageInput {
    /// Input image url./// Input image url./// "https://storage.googleapis.com/falserverless/model_tests/workflow_utils/mask_input.png"
    pub image_url: String,
    /// Color to replace the transparent pixels with/// Color to replace the transparent pixels with/// {"b":128,"g":128,"r":128}
    pub transparent_color: Color,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HEDInput {
    /// URL of the image to process/// URL of the image to process/// "https://storage.googleapis.com/falserverless/model_tests/image_preprocessors/cat.png"
    pub image_url: String,
    /// Whether to use the safe version of the HED detector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe: Option<bool>,
    /// Whether to use the scribble version of the HED detector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scribble: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HEDOutput {
    /// Image with lines detected using the HED detector
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchedMoondreamInput {
    /// List of input prompts and image URLs/// List of input prompts and image URLs/// [{"image_url":"https://github.com/vikhyat/moondream/raw/main/assets/demo-1.jpg","prompt":"What is the girl doing?"}]
    pub inputs: Vec<MoondreamInputParam>,
    /// Maximum number of new tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    /// Model ID to use for inference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// Repetition penalty for sampling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repetition_penalty: Option<f64>,
    /// Temperature for sampling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Top P for sampling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JapaneseOutput {
    /// The generated music/// The generated music/// {"url":"https://fal.media/files/lion/piLhqKO8LJxrWaNg2dVUv_tmpp6eff6zl.wav"}
    pub audio: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LDMTTSOutput {
    /// The generated audio file./// The generated audio file./// {"content_type":"audio/mpeg","duration":24.3,"file_name":"33dd5f07-f834-4080-aaac-4a253ce1660b.mp3","file_size":584109,"url":"https://fal-api-audio-uploads.s3.amazonaws.com/33dd5f07-f834-4080-aaac-4a253ce1660b.mp3"}
    pub audio: AudioFile,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NSFWImageDetectionInput {
    /// Input image url./// Input image url./// "https://storage.googleapis.com/falserverless/model_tests/remove_background/elephant.jpg"
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ItalianRequest {
    pub prompt: String,
    /// Voice ID for the desired voice./// Voice ID for the desired voice./// "if_sara"
    pub voice: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
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
    pub timings: Timings,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputModel {
    /// Camera direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_direction: Option<String>,
    /// Camera movement style
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_style: Option<String>,
    /// Custom technical elements (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_elements: Option<String>,
    /// URL of an image to analyze and incorporate into the video prompt (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// Core concept or thematic input for the video prompt/// Core concept or thematic input for the video prompt/// "A futuristic city at dusk"
    pub input_concept: String,
    /// Model to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Pacing rhythm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pacing: Option<String>,
    /// Length of the prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_length: Option<String>,
    /// Special effects approach
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_effects: Option<String>,
    /// Style of the video prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DreamshaperImageToImageInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://fal-cdn.batuhan-941.workers.dev/files/tiger/IExuP-WICqaIesLZAZPur.jpeg"
    pub image_url: String,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The Dreamshaper model to use./// The Dreamshaper model to use./// "Lykon/dreamshaper-8"
    /// "Lykon/dreamshaper-xl-1-0"
    /// "Lykon/dreamshaper-xl-v2-turbo"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "an island near sea, with seagulls, moon shining over the sea, light house, boats int he background, fish flying over the sea"
    pub prompt: String,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FlowEditOutput {
    /// The generated image file info./// The generated image file info./// {"content_type":"image/png","file_name":"36d3ca4791a647678b2ff01a35c87f5a.png","file_size":423052,"height":1024,"url":"https://storage.googleapis.com/falserverless/model_tests/FlowEdit/aa5c3d028ad04800a54f70f928198d91.png","width":1024}
    pub image: Image,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SAM2ImageOutput {
    /// Segmented image.
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GenFillInput {
    /// Input Image to erase from/// Input Image to erase from/// "https://storage.googleapis.com/falserverless/bria/bria_genfill_img.png"
    pub image_url: String,
    /// The URL of the binary mask image that represents the area that will be cleaned./// The URL of the binary mask image that represents the area that will be cleaned./// "https://storage.googleapis.com/falserverless/bria/bria_genfill_mask.png"
    pub mask_url: String,
    /// The negative prompt you would like to use to generate images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Number of Images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The prompt you would like to use to generate images./// The prompt you would like to use to generate images./// "A red delicious cherry"
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
pub struct ImageToImageControlNetUnionInput {
    /// The URL of the control image./// The URL of the control image./// "https://fal-cdn.batuhan-941.workers.dev/files/rabbit/MiN_j3St9B8esJleCZKMU.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub canny_image_url: Option<String>,
    /// Whether to preprocess the canny image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canny_preprocess: Option<bool>,
    /// The scale of the controlnet conditioning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlnet_conditioning_scale: Option<f64>,
    /// The URL of the control image./// The URL of the control image./// "https://fal-cdn.batuhan-941.workers.dev/files/rabbit/MiN_j3St9B8esJleCZKMU.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_image_url: Option<String>,
    /// Whether to preprocess the depth image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_preprocess: Option<bool>,
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image. Leave it none to automatically infer from the control image./// The size of the generated image. Leave it none to automatically infer from the control image./// null

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://fal-cdn.batuhan-941.workers.dev/files/tiger/IExuP-WICqaIesLZAZPur.jpeg"
    pub image_url: String,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The URL of the control image./// The URL of the control image./// "https://fal-cdn.batuhan-941.workers.dev/files/rabbit/MiN_j3St9B8esJleCZKMU.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_image_url: Option<String>,
    /// Whether to preprocess the normal image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_preprocess: Option<bool>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The URL of the control image./// The URL of the control image./// "https://fal-cdn.batuhan-941.workers.dev/files/rabbit/MiN_j3St9B8esJleCZKMU.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub openpose_image_url: Option<String>,
    /// Whether to preprocess the openpose image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openpose_preprocess: Option<bool>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "Ice fortress, aurora skies, polar wildlife, twilight"
    pub prompt: String,
    /// An id bound to a request, can be used with response to identify the request
    /// itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The URL of the control image./// The URL of the control image./// "https://fal-cdn.batuhan-941.workers.dev/files/rabbit/MiN_j3St9B8esJleCZKMU.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_image_url: Option<String>,
    /// Whether to preprocess the segmentation image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_preprocess: Option<bool>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
    /// The URL of the control image./// The URL of the control image./// "https://fal-cdn.batuhan-941.workers.dev/files/rabbit/MiN_j3St9B8esJleCZKMU.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub teed_image_url: Option<String>,
    /// Whether to preprocess the teed image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teed_preprocess: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CCSROutput {
    /// The generated image file info.
    pub image: Image,
    /// The seed used for the generation.
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BrPortugeseOutput {
    /// The generated music/// The generated music/// {"url":"https://fal.media/files/rabbit/Y9-bWJt5lixo8PTCmncN6_tmpyh7u57oa.wav"}
    pub audio: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AnimateDiffT2VInput {
    /// Number of frames per second to extract from the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<i64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The motions to apply to the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motions: Option<Vec<Option<String>>>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of frames to generate for the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_frames: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the video. Be as descriptive as possible for best results./// The prompt to use for generating the video. Be as descriptive as possible for best results./// "masterpiece, best quality, 1girl, solo, cherry blossoms, hanami, pink flower, white flower, spring season, wisteria, petals, flower, plum blossoms, outdoors, falling petals, white hair, black eyes"
    /// "panda playing a guitar, on a boat, in the ocean, high quality, high quality, ultra HD, realistic"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The size of the video to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_size: Option<VideoSizeProperty>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProUltraTextToImageInput {
    /// The aspect ratio of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatioProperty>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
    pub prompt: String,
    /// Generate less processed, more natural-looking images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<bool>,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_tolerance: Option<String>,
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
pub struct BlurMaskInput {
    /// Input image url./// Input image url./// "https://storage.googleapis.com/falserverless/model_tests/workflow_utils/mask_input.png"
    pub image_url: String,
    /// The radius of the Gaussian blur./// The radius of the Gaussian blur./// 5

    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FooocusOutput {
    /// Whether the generated images contain NSFW concepts.
    pub has_nsfw_concepts: Vec<bool>,
    /// The generated image file info.
    pub images: Vec<Image>,
    /// The time taken for the generation process.
    pub timings: Timings,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToImageInput {
    /// The aspect ratio of the generated image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// A description of what to discourage in the generated images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Number of images to generate (1-4)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The text prompt describing what you want to see/// The text prompt describing what you want to see/// "A serene landscape with mountains reflected in a crystal clear lake at sunset"
    pub prompt: String,
    /// Random seed for reproducible generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TimestepsInput {
    /// Timesteps schedule to be used if 'custom' method is selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array: Option<Vec<Option<i64>>>,
    /// The method to use for the timesteps. If set to 'array', the timesteps will be set based
    /// on the provided timesteps schedule in the `array` field.
    /// Defaults to 'default' which means the scheduler will use the `num_inference_steps` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DiffusionEdgeOutput {
    /// The generated image file info.
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Video {
    /// Audio track information if video has audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Option<AudioTrack>>,
    /// Overall bitrate of the media in bits per second
    pub bitrate: i64,
    /// Codec used to encode the media
    pub codec: String,
    /// Container format of the media file (e.g., 'mp4', 'mov')
    pub container: String,
    /// MIME type of the media file
    pub content_type: String,
    /// Duration of the media in seconds
    pub duration: f64,
    /// URL of the extracted last frame
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_frame_url: Option<String>,
    /// Original filename of the media
    pub file_name: String,
    /// Size of the file in bytes
    pub file_size: i64,
    /// Detailed video format information
    pub format: VideoFormat,
    /// Frames per second
    pub fps: i64,
    /// Total number of frames in the video
    pub frame_count: i64,
    /// Type of media (always 'video')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// Video resolution information
    pub resolution: Resolution,
    /// URL of the extracted first frame
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_frame_url: Option<String>,
    /// Time base used for frame timestamps
    pub timebase: String,
    /// URL where the media file can be accessed
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ObjectOutput {
    /// Generated 3D mesh file
    pub model_mesh: File,
    /// Processing timings
    pub timings: Timings,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InpaintInput {
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// Input image for img2img or inpaint mode/// Input image for img2img or inpaint mode/// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
    pub image_url: String,
    /// Input mask for inpaint mode. Black areas will be preserved, white areas will be inpainted./// Input mask for inpaint mode. Black areas will be preserved, white areas will be inpainted./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
    pub mask_url: String,
    /// URL or HuggingFace ID of the base model to generate the image./// URL or HuggingFace ID of the base model to generate the image./// "diffusers/stable-diffusion-xl-1.0-inpainting-0.1"
    /// "stabilityai/stable-diffusion-xl-base-1.0"
    /// "runwayml/stable-diffusion-v1-5"
    /// "SG161222/Realistic_Vision_V2.0"
    pub model_name: String,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, painting, illustration, (worst quality, low quality, normal quality:2)"
    /// "nsfw, cartoon, (epicnegative:0.9)"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Increasing the amount of steps tells Stable Diffusion that it should take more steps
    /// to generate your final result which can increase the amount of detail in your image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "a photo of a cat"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// 1234

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreativeUpscalerInput {
    /// The URL to the additional embeddings to use for the upscaling. Default is None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_embedding_url: Option<String>,
    /// The scale of the additional LORA model to use for the upscaling. Default is 1.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_lora_scale: Option<f64>,
    /// The URL to the additional LORA model to use for the upscaling. Default is None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_lora_url: Option<String>,
    /// The URL to the base model to use for the upscaling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_model_url: Option<String>,
    /// How much the output can deviate from the original
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creativity: Option<f64>,
    /// How much detail to add
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<f64>,
    /// If set to true, the resulting image will be checked whether it includes any
    /// potentially unsafe content. If it does, it will be replaced with a black
    /// image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checks: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The image to upscale./// The image to upscale./// "https://storage.googleapis.com/falserverless/model_tests/upscale/owl.png"
    /// "https://storage.googleapis.com/falserverless/gallery/blue-bird.jpeg"
    pub image_url: String,
    /// The type of model to use for the upscaling. Default is SD_1_5/// The type of model to use for the upscaling. Default is SD_1_5/// "SD_1_5"
    /// "SDXL"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "blurry, low resolution, bad, ugly, low quality, pixelated, interpolated, compression artifacts, noisey, grainy"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of inference steps to use for generating the image. The more steps
    /// the better the image will be but it will also take longer to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// Allow for large uploads that could take a very long time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_size_limits: Option<bool>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results. If no prompt is provide BLIP2 will be used to generate a prompt./// The prompt to use for generating the image. Be as descriptive as possible for best results. If no prompt is provide BLIP2 will be used to generate a prompt.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// The suffix to add to the prompt. This is useful to add a common ending to all prompts such as 'high quality' etc or embedding tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_suffix: Option<String>,
    /// The scale of the output image. The higher the scale, the bigger the output image will be.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// 42

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// How much to preserve the shape of the original image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape_preservation: Option<f64>,
    /// If set to true, the image will not be processed by the CCSR model before
    /// being processed by the creativity model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_ccsr: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SD3Output {
    /// Whether the generated images contain NSFW concepts.
    pub has_nsfw_concepts: Vec<bool>,
    /// The generated image files info.
    pub images: Vec<Image>,
    /// The number of images generated.
    pub num_images: i64,
    /// The prompt used for generating the image.
    pub prompt: String,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
    pub timings: Timings,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MetadataOutput {
    /// Metadata for the analyzed media file (either Video or Audio)
    pub media: MediaProperty,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SchnellReduxInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to generate an image from./// The URL of the image to generate an image from./// "https://fal.media/files/kangaroo/acQvq-Kmo2lajkgvcEHdv.png"
    pub image_url: String,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
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
pub struct SoteDiffusionInput {
    /// If set to false, the safety checker will be disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// Number of steps to run the first stage for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_stage_steps: Option<i64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "very displeasing, worst quality, monochrome, realistic, oldest"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "newest, extremely aesthetic, best quality, 1girl, solo, pink hair, blue eyes, long hair, looking at viewer, smile, black background, holding a sign, the text on the sign says \"Hello\""
    pub prompt: String,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_stage_guidance_scale: Option<f64>,
    /// Number of steps to run the second stage for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_stage_steps: Option<i64>,
    /// The same seed and the same prompt given to the same version of Stable Cascade
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// If set to true, the image will be returned as base64 encoded string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MochiT2VInput {
    /// Whether to enable prompt expansion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_prompt_expansion: Option<bool>,
    /// The negative prompt for the video./// The negative prompt for the video./// "Blurry, shaky footage"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The prompt to generate a video from./// The prompt to generate a video from./// "A dog running in a field."
    pub prompt: String,
    /// The seed to use for generating the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct T2VLiveOutput {
    /// The generated video/// The generated video/// {"url":"https://fal.media/files/monkey/EbJRdZfaJbNiJBUvPta3c_output.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImagePrompt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_at: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub ty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AnimateDiffV2VOutput {
    /// Seed used for generating the video.
    pub seed: i64,
    /// Generated video file./// Generated video file./// {"url":"https://fal-cdn.batuhan-941.workers.dev/files/koala/5Cb_6P_s9wW8f8-g9c4yj.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Ben2OutputImage {
    /// The output image after background removal./// The output image after background removal./// {"content_type":"image/png","file_name":"zrZNETpI_ul2jonraqpxN_a57c3f3825d9418f8b3d39cde87c3310.png","file_size":423052,"height":512,"url":"https://storage.googleapis.com/falserverless/gallery/Ben2/zrZNETpI_ul2jonraqpxN_a57c3f3825d9418f8b3d39cde87c3310.png","width":512}
    pub image: Image,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToImageSD15Input {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"
    /// "ugly, deformed"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "photo of a rhino dressed suit and tie sitting at a table in a bar with a bar stools, award winning photography, Elke vogelsang"
    /// "Photo of a classic red mustang car parked in las vegas strip at night"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
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
pub struct Frame {
    /// URL of the frame
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpscaleInput {
    /// The URL of the image to be upscaled. Must be in PNG format./// The URL of the image to be upscaled. Must be in PNG format./// "https://storage.googleapis.com/falserverless/model_tests/recraft/recraft-upscaler-1.jpeg"
    pub image_url: String,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LightningModelsInpaintingInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
    pub image_url: String,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The URL of the mask to use for inpainting./// The URL of the mask to use for inpainting./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
    pub mask_url: String,
    /// The Lightning model to use./// The Lightning model to use./// "Lykon/dreamshaper-xl-lightning"
    /// "SG161222/RealVisXL_V4.0_Lightning"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "a tiger sitting on a park bench"
    pub prompt: String,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// Scheduler / sampler to use for the image denoising process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ComfyInput {
    /// Disable saving prompt metadata in files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<HashMap<String, serde_json::Value>>,
    pub prompt: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ControlLoraWeight {
    /// URL of the image to be used as the control image.
    pub control_image_url: String,
    /// URL or the path to the LoRA weights.
    pub path: String,
    /// Type of preprocessing to apply to the input image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preprocess: Option<String>,
    /// The scale of the LoRA weight. This is used to scale the LoRA weight
    /// before merging it with the base model. Providing a dictionary as {"layer_name":layer_scale} allows per-layer lora scale settings. Layers with no scale provided will have scale 1.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<ScaleProperty>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToImageTurboInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The name of the model to use./// The name of the model to use./// "stabilityai/sdxl-turbo"
    /// "stabilityai/sd-turbo"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"
    /// "ugly, deformed"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "A cinematic shot of a baby racoon wearing an intricate italian priest robe."
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
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
pub struct ZonosOutput {
    /// The generated audio
    pub audio: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxFinetuneOutput {
    /// References your specific model
    pub finetune_id: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EditImageInput {
    /// Whether to expand the prompt with MagicPrompt functionality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The image URL to generate an image from. Needs to match the dimensions of the mask./// The image URL to generate an image from. Needs to match the dimensions of the mask./// "https://storage.googleapis.com/falserverless/flux-lora/example-images/knight.jpeg"
    pub image_url: String,
    /// The mask URL to inpaint the image. Needs to match the dimensions of the input image./// The mask URL to inpaint the image. Needs to match the dimensions of the input image./// "https://storage.googleapis.com/falserverless/flux-lora/example-images/mask_knight.jpeg"
    pub mask_url: String,
    /// The prompt to fill the masked part of the image./// The prompt to fill the masked part of the image./// "A knight in shining armour holding a greatshield with \"FAL\" on it"
    pub prompt: String,
    /// Seed for the random number generator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The style of the generated image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageExpansionOutput {
    /// The generated image/// The generated image/// {"content_type":"image/png","file_name":"afa402a35ea742cdb5c3e219b2b19bfb.png","file_size":1471342,"height":674,"url":"https://v3.fal.media/files/koala/8np-spgxxG-I1r3cjthRV_afa402a35ea742cdb5c3e219b2b19bfb.png","width":1200}
    pub image: Image,
    /// Seed value used for generation.
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DiffInput {
    /// URL of change map./// URL of change map./// "https://fal.media/files/zebra/Wh4IYAiAAcVbuZ8M9ZMSn.jpeg"
    pub change_map_image_url: String,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// URL of image to use as initial image./// URL of image to use as initial image./// "https://fal.media/files/koala/h6a7KK2Ie_inuGbdartoX.jpeg"
    pub image_url: String,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Tree of life under the sea, ethereal, glittering, lens flares, cinematic lighting, artwork by Anna Dittmann & Carne Griffiths, 8k, unreal engine 5, hightly detailed, intricate detailed."
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The strength to use for image-to-image. 1.0 is completely remakes the image while 0.0 preserves the original.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InpaintingTurboInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
    pub image_url: String,
    /// The URL of the mask to use for inpainting./// The URL of the mask to use for inpainting./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
    pub mask_url: String,
    /// The name of the model to use./// The name of the model to use./// "stabilityai/sdxl-turbo"
    /// "stabilityai/sd-turbo"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "a tiger sitting on a park bench"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DDColorInput {
    /// URL of image to be used for relighting/// URL of image to be used for relighting/// "https://storage.googleapis.com/falserverless/gallery/Screenshot%202025-02-26%20154226.png"
    pub image_url: String,
    /// seed to be used for generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct V1TextToVideoRequest {
    /// Advanced Camera control parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_camera_control: Option<Option<CameraControl>>,
    /// The aspect ratio of the generated video frame
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// Camera control parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_control: Option<String>,
    /// The duration of the generated video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InpaintingLightningInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
    pub image_url: String,
    /// The URL of the mask to use for inpainting./// The URL of the mask to use for inpainting./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
    pub mask_url: String,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<String>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "a tiger sitting on a park bench"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PolygonOutput {
    /// List of polygons
    pub polygons: Vec<Polygon>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LivePortraitInput {
    /// Amount to open mouth in 'aaa' shape
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aaa: Option<f64>,
    /// Batch size for the model. The larger the batch size, the faster the model will run, but the more memory it will consume.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i64>,
    /// Amount to blink the eyes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blink: Option<f64>,
    /// Size of the output image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dsize: Option<i64>,
    /// Amount to shape mouth in 'eee' position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eee: Option<f64>,
    /// Whether to enable the safety checker. If enabled, the model will check if the input image contains a face before processing it.
    /// The safety checker will process the input image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// Amount to raise or lower eyebrows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eyebrow: Option<f64>,
    /// Whether to crop the source portrait to the face-cropping space.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_do_crop: Option<bool>,
    /// Whether to conduct the rotation when flag_do_crop is True.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_do_rot: Option<bool>,
    /// Whether to enable eye retargeting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_eye_retargeting: Option<bool>,
    /// Whether to enable lip retargeting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_lip_retargeting: Option<bool>,
    /// Whether to set the lip to closed state before animation. Only takes effect when flag_eye_retargeting and flag_lip_retargeting are False.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_lip_zero: Option<bool>,
    /// Whether to paste-back/stitch the animated face cropping from the face-cropping space to the original image space.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_pasteback: Option<bool>,
    /// Whether to use relative motion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_relative: Option<bool>,
    /// Whether to enable stitching. Recommended to set to True.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_stitching: Option<bool>,
    /// URL of the image to be animated/// URL of the image to be animated/// "https://storage.googleapis.com/falserverless/model_tests/live-portrait/XKEmk3mAzGHUjK3qqH-UL.jpeg"
    pub image_url: String,
    /// Amount to move pupils horizontally
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pupil_x: Option<f64>,
    /// Amount to move pupils vertically
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pupil_y: Option<f64>,
    /// Amount to rotate the face in pitch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_pitch: Option<f64>,
    /// Amount to rotate the face in roll
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_roll: Option<f64>,
    /// Amount to rotate the face in yaw
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_yaw: Option<f64>,
    /// Scaling factor for the face crop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    /// Amount to smile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smile: Option<f64>,
    /// URL of the video to drive the lip syncing./// URL of the video to drive the lip syncing./// "https://storage.googleapis.com/falserverless/model_tests/live-portrait/liveportrait-example.mp4"
    pub video_url: String,
    /// Horizontal offset ratio for face crop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vx_ratio: Option<f64>,
    /// Vertical offset ratio for face crop. Positive values move up, negative values move down.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vy_ratio: Option<f64>,
    /// Amount to wink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wink: Option<f64>,
    /// Amount to shape mouth in 'woo' position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub woo: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ControlNeXtOutput {
    /// The generated video.
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TeeDInput {
    /// URL of the image to process/// URL of the image to process/// "https://storage.googleapis.com/falserverless/model_tests/image_preprocessors/cat.png"
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BGRemoveOutput {
    /// The generated image/// The generated image/// {"content_type":"image/png","file_name":"070c731993e949d993c10ef6283d335d.png","file_size":1076276,"height":1024,"url":"https://v3.fal.media/files/tiger/GQEMNjRyxSoza7N8LPPqb_070c731993e949d993c10ef6283d335d.png","width":1024}
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RemoveBackgroundInput {
    /// If set to true, the resulting image be cropped to a bounding box around the subject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop_to_bbox: Option<bool>,
    /// Input image url./// Input image url./// "https://storage.googleapis.com/falserverless/model_tests/remove_background/elephant.jpg"
    pub image_url: String,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StyleReferenceInput {
    /// The base style of the generated images, this topic is covered above.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_style: Option<String>,
    /// URL to zip archive with images, use PNG format. Maximum 5 images are allowed.
    pub images_data_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LineartOutput {
    /// Image with edges detected using the Canny algorithm
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ZoeInput {
    /// URL of the image to process/// URL of the image to process/// "https://storage.googleapis.com/falserverless/model_tests/image_preprocessors/cat.png"
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ZoeOutput {
    /// Image with depth map
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LDMTTSInput {
    /// S3 URI of the autoregressive (AR) model./// S3 URI of the autoregressive (AR) model./// null

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ar: Option<String>,
    /// The dialogue text with turn prefixes to distinguish speakers./// The dialogue text with turn prefixes to distinguish speakers./// "Speaker 1: Hey, did you catch the game last night?\nSpeaker 2: Of course! What a match—it had me on the edge of my seat.\nSpeaker 1: Same here! That last-minute goal was unreal. Who's your MVP?\nSpeaker 2: Gotta be the goalie. Those saves were unbelievable.\nSpeaker 1: Absolutely. Saved the day, literally! Are you planning to watch the next game?\nSpeaker 2: Oh, you bet. I’m already stocked up on snacks!\n"
    pub input: String,
    /// S3 URI of the AR LoRA model./// S3 URI of the AR LoRA model./// null

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lora: Option<String>,
    /// The format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    /// An integer number greater than or equal to 0. If equal to null or not provided, a random seed will be used. Useful to control the reproducibility of the generated audio. Assuming all other properties didn't change, a fixed seed should always generate the exact same audio file./// An integer number greater than or equal to 0. If equal to null or not provided, a random seed will be used. Useful to control the reproducibility of the generated audio. Assuming all other properties didn't change, a fixed seed should always generate the exact same audio file./// null

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// S3 URI of the vocoder model./// S3 URI of the vocoder model./// null

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocoder: Option<String>,
    /// A list of voice definitions for each speaker in the dialogue. Must be between 1 and 2 voices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voices: Option<Vec<Option<LDMVoiceInput>>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchMoonDreamOutput {
    /// URL to the generated captions JSON file containing filename-caption pairs.
    pub captions_file: File,
    /// List of generated captions
    pub outputs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MiDaSOutput {
    /// Image with MiDaS depth map
    pub depth_map: Image,
    /// Image with MiDaS normal map
    pub normal_map: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShrinkMaskOutput {
    /// The mask/// The mask/// {"content_type":"image/png","height":700,"url":"https://storage.googleapis.com/falserverless/model_tests/workflow_utils/shrink_mask_output.png","width":610}
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HindiOutput {
    /// The generated music/// The generated music/// {"url":"https://fal.media/files/elephant/3sGUskl1AFG4TN_NAinO8_tmpdq_1m8og.wav"}
    pub audio: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SamOutput {
    /// Image with SAM segmentation map
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InpaintingFooocusInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, a smaller model will try to refine the output after it was processed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_refiner: Option<bool>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The rescale factor for the CFG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_rescale: Option<f64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
    pub image_url: String,
    /// The URL of the mask to use for inpainting./// The URL of the mask to use for inpainting./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
    pub mask_url: String,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "a tiger sitting on a park bench"
    pub prompt: String,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TeedInput {
    /// Input image url./// Input image url./// "https://storage.googleapis.com/falserverless/model_tests/retoucher/GGsAolHXsAA58vn.jpeg"
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TransparentImageToMaskInput {
    /// Input image url./// Input image url./// "https://storage.googleapis.com/falserverless/model_tests/workflow_utils/transparent_image_to_mask_input.png"
    pub image_url: String,
    /// The threshold to convert the image to a mask./// The threshold to convert the image to a mask./// 128

    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AnimateDiffT2VOutput {
    /// Seed used for generating the video.
    pub seed: i64,
    /// Generated video file./// Generated video file./// {"url":"https://fal-cdn.batuhan-941.workers.dev/files/kangaroo/DSrFBOk9XXIplm_kukI4n.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ScribbleInput {
    /// URL of the image to process/// URL of the image to process/// "https://storage.googleapis.com/falserverless/model_tests/image_preprocessors/cat.png"
    pub image_url: String,
    /// The model to use for the Scribble detector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Whether to use the safe version of the Scribble detector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JapaneseRequest {
    pub prompt: String,
    /// Voice ID for the desired voice./// Voice ID for the desired voice./// "jf_alpha"
    pub voice: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LineartInput {
    /// Whether to use the coarse model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coarse: Option<bool>,
    /// URL of the image to process/// URL of the image to process/// "https://storage.googleapis.com/falserverless/model_tests/image_preprocessors/cat.png"
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DetectionInput {
    /// Text description of what to detect/// Text description of what to detect/// "Person"
    pub detection_prompt: String,
    /// Image URL to be processed/// Image URL to be processed/// "https://llava-vl.github.io/static/images/monalisa.jpg"
    pub image_url: String,
    /// Type of detection to perform
    pub task_type: String,
    /// Whether to use ensemble for gaze detection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_ensemble: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AudioIsolationRequest {
    /// URL of the audio file to isolate voice from/// URL of the audio file to isolate voice from/// "https://v3.fal.media/files/zebra/zJL_oRY8h5RWwjoK1w7tx_output.mp3"
    pub audio_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PoseTransferOutput {
    /// Whether the image contains NSFW concepts.
    pub has_nsfw_concepts: bool,
    /// The output image./// The output image./// {"content_type":"image/jpeg","height":1024,"url":"https://fal.media/files/tiger/y6ZwaYdP9Q92FnsJcSbYz.png","width":768}
    pub image: Image,
    /// The seed for the inference.
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShrinkMaskInput {
    /// Input image url./// Input image url./// "https://storage.googleapis.com/falserverless/model_tests/workflow_utils/mask_input.png"
    pub image_url: String,
    /// The number of pixels to shrink the mask./// The number of pixels to shrink the mask./// 5

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixels: Option<i64>,
    /// The threshold to convert the image to a mask. 0-255./// The threshold to convert the image to a mask. 0-255./// 128

    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MiniCPMV26VideoInput {
    /// Prompt to be used for the video description/// Prompt to be used for the video description/// "What is she doing? Describe it detailed way."
    pub prompt: String,
    /// URL of the video to be analyzed/// URL of the video to be analyzed/// "https://storage.googleapis.com/falserverless/model_tests/musepose/dance.mp4"
    pub video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BGReplaceInput {
    /// Whether to use the fast model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fast: Option<bool>,
    /// Input Image to erase from/// Input Image to erase from/// "https://storage.googleapis.com/falserverless/bria/bria_bg_replace_fg.jpg"
    pub image_url: String,
    /// The negative prompt you would like to use to generate images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Number of Images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The prompt you would like to use to generate images./// The prompt you would like to use to generate images./// "Lilypad on a river"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// The URL of the reference image to be used for generating the new background. Use "" to leave empty. Either ref_image_url or bg_prompt has to be provided but not both. If both ref_image_url and ref_image_file are provided, ref_image_url will be used. Accepted formats are jpeg, jpg, png, webp./// The URL of the reference image to be used for generating the new background. Use "" to leave empty. Either ref_image_url or bg_prompt has to be provided but not both. If both ref_image_url and ref_image_file are provided, ref_image_url will be used. Accepted formats are jpeg, jpg, png, webp./// "https://storage.googleapis.com/falserverless/bria/bria_bg_replace_bg.jpg"

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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SpanishRequest {
    pub prompt: String,
    /// Voice ID for the desired voice./// Voice ID for the desired voice./// "ef_dora"
    pub voice: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StepFunT2VResponse {
    /// The seed used for generating the video.
    pub seed: i64,
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SAM2VideoRLEInput {
    /// Apply the mask on the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_mask: Option<bool>,
    /// Coordinates for boxes/// Coordinates for boxes/// [{"frame_index":0,"x_max":500,"x_min":300,"y_max":400,"y_min":0}]

    #[serde(skip_serializing_if = "Option::is_none")]
    pub box_prompts: Option<Vec<Option<BoxPrompt>>>,
    /// List of prompts to segment the video/// List of prompts to segment the video/// [{"frame_index":0,"label":1,"x":210,"y":350},{"frame_index":0,"label":1,"x":250,"y":220}]

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<Vec<Option<PointPrompt>>>,
    /// The URL of the video to be segmented./// The URL of the video to be segmented./// "https://drive.google.com/uc?id=1iOFYbNITYwrebBBp9kaEGhBndFSRLz8k"
    pub video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HunyuanI2VResponse {
    /// The seed used for generating the video.
    pub seed: i64,
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct V3TTSInput {
    /// The text to be converted to speech./// The text to be converted to speech./// "The quick brown fox jumped over the lazy dog."
    pub input: String,
    /// The format of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    /// An integer number greater than or equal to 0. If equal to null or not provided, a random seed will be used. Useful to control the reproducibility of the generated audio. Assuming all other properties didn't change, a fixed seed should always generate the exact same audio file./// An integer number greater than or equal to 0. If equal to null or not provided, a random seed will be used. Useful to control the reproducibility of the generated audio. Assuming all other properties didn't change, a fixed seed should always generate the exact same audio file./// null

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The unique ID of a PlayHT or Cloned Voice, or a name from the available presets./// The unique ID of a PlayHT or Cloned Voice, or a name from the available presets./// "Jennifer (English (US)/American)"
    /// "Dexter (English (US)/American)"
    /// "Ava (English (AU)/Australian)"
    /// "Tilly (English (AU)/Australian)"
    /// "Charlotte (Advertising) (English (CA)/Canadian)"
    /// "Charlotte (Meditation) (English (CA)/Canadian)"
    /// "Cecil (English (GB)/British)"
    /// "Sterling (English (GB)/British)"
    /// "Cillian (English (IE)/Irish)"
    /// "Madison (English (IE)/Irish)"
    /// "Ada (English (ZA)/South african)"
    /// "Furio (English (IT)/Italian)"
    /// "Alessandro (English (IT)/Italian)"
    /// "Carmen (English (MX)/Mexican)"
    /// "Sumita (English (IN)/Indian)"
    /// "Navya (English (IN)/Indian)"
    /// "Baptiste (English (FR)/French)"
    /// "Lumi (English (FI)/Finnish)"
    /// "Ronel Conversational (Afrikaans/South african)"
    /// "Ronel Narrative (Afrikaans/South african)"
    /// "Abdo Conversational (Arabic/Arabic)"
    /// "Abdo Narrative (Arabic/Arabic)"
    /// "Mousmi Conversational (Bengali/Bengali)"
    /// "Mousmi Narrative (Bengali/Bengali)"
    /// "Caroline Conversational (Portuguese (BR)/Brazilian)"
    /// "Caroline Narrative (Portuguese (BR)/Brazilian)"
    /// "Ange Conversational (French/French)"
    /// "Ange Narrative (French/French)"
    /// "Anke Conversational (German/German)"
    /// "Anke Narrative (German/German)"
    /// "Bora Conversational (Greek/Greek)"
    /// "Bora Narrative (Greek/Greek)"
    /// "Anuj Conversational (Hindi/Indian)"
    /// "Anuj Narrative (Hindi/Indian)"
    /// "Alessandro Conversational (Italian/Italian)"
    /// "Alessandro Narrative (Italian/Italian)"
    /// "Kiriko Conversational (Japanese/Japanese)"
    /// "Kiriko Narrative (Japanese/Japanese)"
    /// "Dohee Conversational (Korean/Korean)"
    /// "Dohee Narrative (Korean/Korean)"
    /// "Ignatius Conversational (Malay/Malay)"
    /// "Ignatius Narrative (Malay/Malay)"
    /// "Adam Conversational (Polish/Polish)"
    /// "Adam Narrative (Polish/Polish)"
    /// "Andrei Conversational (Russian/Russian)"
    /// "Andrei Narrative (Russian/Russian)"
    /// "Aleksa Conversational (Serbian/Serbian)"
    /// "Aleksa Narrative (Serbian/Serbian)"
    /// "Carmen Conversational (Spanish/Spanish)"
    /// "Patricia Conversational (Spanish/Spanish)"
    /// "Aiken Conversational (Tagalog/Filipino)"
    /// "Aiken Narrative (Tagalog/Filipino)"
    /// "Katbundit Conversational (Thai/Thai)"
    /// "Katbundit Narrative (Thai/Thai)"
    /// "Ali Conversational (Turkish/Turkish)"
    /// "Ali Narrative (Turkish/Turkish)"
    /// "Sahil Conversational (Urdu/Pakistani)"
    /// "Sahil Narrative (Urdu/Pakistani)"
    /// "Mary Conversational (Hebrew/Israeli)"
    /// "Mary Narrative (Hebrew/Israeli)"
    pub voice: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpscaleImageInput {
    /// The detail of the upscaled image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<i64>,
    /// Whether to expand the prompt with MagicPrompt functionality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The image URL to upscale/// The image URL to upscale/// "https://fal.media/files/monkey/e6RtJf_ue0vyWzeiEmTby.png"
    pub image_url: String,
    /// The prompt to upscale the image with
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// The resemblance of the upscaled image to the original image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resemblance: Option<i64>,
    /// Seed for the random number generator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DevReduxInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to generate an image from./// The URL of the image to generate an image from./// "https://fal.media/files/kangaroo/acQvq-Kmo2lajkgvcEHdv.png"
    pub image_url: String,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
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
pub struct ControlNet {
    /// The scale of the control net weight. This is used to scale the control net weight
    /// before merging it with the base model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditioning_scale: Option<f64>,
    /// optional URL to the controlnet config.json file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_url: Option<String>,
    /// The percentage of the image to end applying the controlnet in terms of the total timesteps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_percentage: Option<f64>,
    /// URL of the image to be used as the control net.
    pub image_url: String,
    /// The index of the IP adapter to be applied to the controlnet. This is only needed for InstantID ControlNets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_adapter_index: Option<i64>,
    /// The mask to use for the controlnet. When using a mask, the control image size and the mask size must be the same and divisible by 32.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_url: Option<String>,
    /// URL or the path to the control net weights.
    pub path: String,
    /// The percentage of the image to start applying the controlnet in terms of the total timesteps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_percentage: Option<f64>,
    /// The optional variant if a Hugging Face repo key is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SoundEffectRequest {
    /// Duration in seconds (0.5-22). If None, optimal duration will be determined from prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<DurationSecondsProperty>,
    /// How closely to follow the prompt (0-1). Higher values mean less variation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_influence: Option<f64>,
    /// The text describing the sound effect to generate/// The text describing the sound effect to generate/// "Spacious braam suitable for high-impact movie trailer moments"
    /// "A gentle wind chime tinkling in a soft breeze"
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SpeechToTextRequest {
    /// URL of the audio file to transcribe/// URL of the audio file to transcribe/// "https://v3.fal.media/files/zebra/zJL_oRY8h5RWwjoK1w7tx_output.mp3"
    pub audio_url: String,
    /// Whether to annotate who is speaking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diarize: Option<bool>,
    /// Language code of the audio/// Language code of the audio/// "eng"
    /// "spa"
    /// "fra"
    /// "deu"
    /// "jpn"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCodeProperty>,
    /// Tag audio events like laughter, applause, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_audio_events: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubjectCustomizeInput {
    /// A description of what to discourage in the generated images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Number of images to generate (1-4)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The text prompt describing what you want to see, using [1] to reference the subject/// The text prompt describing what you want to see, using [1] to reference the subject/// "Image of [1] in sunglasses posing as an astronaut in the moon"
    pub prompt: String,
    /// 1-4 reference images of the subject to customize/// 1-4 reference images of the subject to customize/// [{"image_url":"https://raw.githubusercontent.com/google/dreambooth/refs/heads/main/dataset/dog/01.jpg"},{"image_url":"https://raw.githubusercontent.com/google/dreambooth/refs/heads/main/dataset/dog/02.jpg"}]
    pub reference_images: Vec<ReferenceImage>,
    /// Random seed for reproducible generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Optional description of the subject in the reference images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_description: Option<String>,
    /// Type of subject in the reference images/// Type of subject in the reference images/// "animal"
    pub subject_type: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProPlusTextToImageInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_tolerance: Option<String>,
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
pub struct ConformerOutput {
    /// The generated image file info./// The generated image file info./// {"content_type":"image/png","file_name":"36d3ca4791a647678b2ff01a35c87f5a.png","file_size":423052,"height":512,"url":"https://storage.googleapis.com/falserverless/model_tests/codeformer/codeformer_restored_1.jpeg","width":512}
    pub image: Image,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReferenceFace {
    /// URL of the reference face image
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CATVTONInput {
    /// Type of the Cloth to be tried on.
    ///
    /// Options:
    /// upper: Upper body cloth
    /// lower: Lower body cloth
    /// overall: Full body cloth
    /// inner: Inner cloth, like T-shirt inside a jacket
    /// outer: Outer cloth, like a jacket over a T-shirt/// Type of the Cloth to be tried on.
    ///
    /// Options:
    /// upper: Upper body cloth
    /// lower: Lower body cloth
    /// overall: Full body cloth
    /// inner: Inner cloth, like T-shirt inside a jacket
    /// outer: Outer cloth, like a jacket over a T-shirt/// "upper"
    /// "lower"
    /// "overall"
    /// "inner"
    /// "outer"
    pub cloth_type: String,
    /// Url to the garment image./// Url to the garment image./// "https://storage.googleapis.com/falserverless/catvton/tshirt.jpg"
    pub garment_image_url: String,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// Url for the human image./// Url for the human image./// "https://storage.googleapis.com/falserverless/catvton/man5.jpg"
    pub human_image_url: String,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The same seed and the same input given to the same version of the model
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LivePortraitImageOutput {
    /// The generated image file.
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SAM2ImageInput {
    /// Coordinates for boxes/// Coordinates for boxes/// [{"x_max":700,"x_min":425,"y_max":875,"y_min":600}]

    #[serde(skip_serializing_if = "Option::is_none")]
    pub box_prompts: Option<Vec<Option<BoxPrompt>>>,
    /// URL of the image to be segmented/// URL of the image to be segmented/// "https://raw.githubusercontent.com/facebookresearch/segment-anything-2/main/notebooks/images/truck.jpg"
    pub image_url: String,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// List of prompts to segment the image/// List of prompts to segment the image/// [{"label":1,"x":500,"y":375}]

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<Vec<Option<PointPrompt>>>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Ray2ImageToVideoRequest {
    /// The aspect ratio of the generated video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// The duration of the generated video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Final image to end the video with. Can be used together with image_url.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_image_url: Option<String>,
    /// Initial image to start the video from. Can be used together with end_image_url./// Initial image to start the video from. Can be used together with end_image_url./// "https://fal.media/files/elephant/8kkhB12hEZI2kkbU8pZPA_test.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// Whether the video should loop (end of video is blended with the beginning)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "loop")]
    pub r#loop: Option<bool>,
    pub prompt: String,
    /// The resolution of the generated video (720p costs 2x more)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PhotoLoraT2IInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The LoRAs to use for the image generation. You can use any number of LoRAs
    /// and they will be merged together to generate the final image./// The LoRAs to use for the image generation. You can use any number of LoRAs
    /// and they will be merged together to generate the final image.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// LoRA Scale of the photo lora model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_lora_scale: Option<f64>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
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
pub struct DocResInput {
    /// URL of image to be used for relighting/// URL of image to be used for relighting/// "https://storage.googleapis.com/falserverless/docres_ckpt/218_in.png"
    pub image_url: String,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Task to perform
    pub task: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VideoEffectsRequest {
    /// The duration of the generated video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// The effect scene to use for the video generation/// The effect scene to use for the video generation/// "hug"
    pub effect_scene: String,
    /// URL of images./// URL of images./// ["https://storage.googleapis.com/falserverless/juggernaut_examples/VHXMavzPyI27zi6JseyL4.png","https://storage.googleapis.com/falserverless/juggernaut_examples/QEW5VrzccxGva7mPfEXjf.png"]

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_image_urls: Option<Vec<Option<String>>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SadTalkerOutput {
    /// URL of the generated video
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MetadataInput {
    /// Whether to extract the start and end frames for videos. Note that when true the request will be slower.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_frames: Option<bool>,
    /// URL of the media file (video or audio) to analyze
    pub media_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MultiConditioningVideoInput {
    /// Aspect ratio of the generated video (16:9 or 9:16).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// Whether to expand the prompt using the model's own capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// URL of images to use as conditioning/// URL of images to use as conditioning/// [{"image_url":"https://storage.googleapis.com/falserverless/model_tests/ltx/NswO1P8sCLzrh1WefqQFK_9a6bdbfa54b944c9a770338159a113fd.jpg","start_frame_num":0},{"image_url":"https://storage.googleapis.com/falserverless/model_tests/ltx/YAPOGvmS2tM_Krdp7q6-d_267c97e017c34f679844a4477dfcec38.jpg","start_frame_num":120}]

    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<Option<ImageConditioningInput>>>,
    /// Negative prompt for generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Number of inference steps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// Text prompt to guide generation/// Text prompt to guide generation/// "\n            A vibrant, abstract composition featuring a person with outstretched arms, rendered in a kaleidoscope of colors against a deep, dark background. The figure is composed of intricate, swirling patterns reminiscent of a mosaic, with hues of orange, yellow, blue, and green that evoke the style of artists such as Wassily Kandinsky or Bridget Riley. \n\nThe camera zooms into the face striking portrait of a man, reimagined through the lens of old-school video-game graphics. The subject's face is rendered in a kaleidoscope of colors, with bold blues and reds set against a vibrant yellow backdrop. His dark hair is pulled back, framing his profile in a dramatic pose\n        "
    pub prompt: String,
    /// Resolution of the generated video (480p or 720p).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// Random seed for generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Videos to use as conditioning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<Vec<Option<VideoConditioningInput>>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WanT2VResponse {
    /// The seed used for generation.
    pub seed: i64,
    /// The generated video file./// The generated video file./// {"url":"https://storage.googleapis.com/falserverless/web-examples/wan/t2v.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EraserInput {
    /// Input Image to erase from/// Input Image to erase from/// "https://storage.googleapis.com/falserverless/bria/bria_eraser_img.png"
    pub image_url: String,
    /// You can use this parameter to specify the type of the input mask from the list. 'manual' opttion should be used in cases in which the mask had been generated by a user (e.g. with a brush tool), and 'automatic' mask type should be used when mask had been generated by an algorithm like 'SAM'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_type: Option<String>,
    /// The URL of the binary mask image that represents the area that will be cleaned./// The URL of the binary mask image that represents the area that will be cleaned./// "https://storage.googleapis.com/falserverless/bria/bria_eraser_mask.png"
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
pub struct FaceDetection {
    /// Bounding box of the face./// Bounding box of the face./// [0,0,100,100]
    pub bbox: Vec<i64>,
    /// Confidence score of the detection./// Confidence score of the detection./// 0.9
    pub det_score: f64,
    /// Embedding of the face./// Embedding of the face./// ""
    pub embedding_file: File,
    /// Keypoints of the face./// Keypoints of the face.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kps: Option<Vec<Option<Vec<Option<i64>>>>>,
    /// Keypoints of the face on the image./// Keypoints of the face on the image.
    pub kps_image: Image,
    /// Either M or F if available./// Either M or F if available./// "M"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToVideoRequest {
    /// The aspect ratio of the generated video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// The duration of the generated video in seconds. 8s videos cost double. 1080p videos are limited to 5 seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Negative prompt to be used for the generation/// Negative prompt to be used for the generation/// "blurry, low quality, low resolution, pixelated, noisy, grainy, out of focus, poorly lit, poorly exposed, poorly composed, poorly framed, poorly cropped, poorly color corrected, poorly color graded"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    pub prompt: String,
    /// The resolution of the generated video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same video every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The style of the generated video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InpaintTurboInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image. Defaults to landscape_4_3 if no controlnet has been passed, otherwise defaults to the size of the controlnet conditioning image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// URL of Image for inpainting/// URL of Image for inpainting/// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
    pub image_url: String,
    /// URL of mask image for inpainting./// URL of mask image for inpainting./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
    pub mask_image_url: String,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// ""

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Tiger sitting on a bench."
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Strength for Image-to-Image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PhotoLoraI2IInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// URL of image to use for inpainting. or img2img/// URL of image to use for inpainting. or img2img/// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
    pub image_url: String,
    /// The LoRAs to use for the image generation. You can use any number of LoRAs
    /// and they will be merged together to generate the final image./// The LoRAs to use for the image generation. You can use any number of LoRAs
    /// and they will be merged together to generate the final image.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// LoRA Scale of the photo lora model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_lora_scale: Option<f64>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "A photo of a lion sitting on a stone bench"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The strength to use for inpainting/image-to-image. Only used if the image_url is provided. 1.0 is completely remakes the image while 0.0 preserves the original.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageToVideoInput {
    /// The aspect ratio of the generated video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// URL of the input image to animate. Should be 720p or higher resolution./// URL of the input image to animate. Should be 720p or higher resolution./// "https://fal.media/files/elephant/6fq8JDSjb1osE_c3J_F2H.png"
    pub image_url: String,
    /// The text prompt describing how the image should be animated/// The text prompt describing how the image should be animated/// "A lego chef cooking eggs"
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct I2VLiveOutput {
    /// The generated video/// The generated video/// {"url":"https://fal.media/files/monkey/bkT4T4uLOXr0jDsIMlNd5_output.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToVideoLiveRequest {
    pub prompt: String,
    /// Whether to use the model's prompt optimizer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_optimizer: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SchnellTextToImageInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
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
pub struct HTTPValidationError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Vec<Option<ValidationError>>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WanI2VResponse {
    /// The seed used for generation.
    pub seed: i64,
    /// The generated video file./// The generated video file./// {"url":"https://v3.fal.media/files/elephant/Nj4jZupkZvR7g0QkNueJZ_video-1740522225.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InsightfaceInput {
    /// Size of the detection./// Size of the detection./// 640

    #[serde(skip_serializing_if = "Option::is_none")]
    pub det_size_height: Option<i64>,
    /// Size of the detection./// Size of the detection./// 640

    #[serde(skip_serializing_if = "Option::is_none")]
    pub det_size_width: Option<i64>,
    /// Input image url./// Input image url./// "https://storage.googleapis.com/falserverless/model_tests/retoucher/GGsAolHXsAA58vn.jpeg"
    pub image_url: String,
    /// Maximum number of faces to detect./// Maximum number of faces to detect./// 1

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_face_num: Option<i64>,
    /// URL of the model weights./// URL of the model weights./// "buffalo_l"
    /// "antelopev2"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_url: Option<String>,
    /// Sorting of the faces./// Sorting of the faces./// "largest-to-smallest"
    /// "smallest-to-largest"
    /// "left-to-right"
    /// "right-to-left"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorting: Option<String>,
    /// Whether to run in sync mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
    /// Threshold for the edge map./// Threshold for the edge map./// 0.5

    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ScribbleOutput {
    /// Image with lines detected using the Scribble detector
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageToImageControlNetInput {
    /// The URL of the control image./// The URL of the control image./// "https://avatars.githubusercontent.com/u/74778219"
    pub control_image_url: String,
    /// The scale of the controlnet conditioning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlnet_conditioning_scale: Option<f64>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image. Leave it none to automatically infer from the control image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://fal-cdn.batuhan-941.workers.dev/files/tiger/IExuP-WICqaIesLZAZPur.jpeg"
    pub image_url: String,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "Ice fortress, aurora skies, polar wildlife, twilight"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SadTalkerRefVideoInput {
    /// URL of the driven audio/// URL of the driven audio/// "https://storage.googleapis.com/falserverless/model_tests/sadtalker/deyu.wav"
    pub driven_audio_url: String,
    /// The scale of the expression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_scale: Option<f64>,
    /// The type of face enhancer to use/// The type of face enhancer to use/// null

    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_enhancer: Option<String>,
    /// The resolution of the face model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_resolution: Option<String>,
    /// The style of the pose
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose_style: Option<i64>,
    /// The type of preprocessing to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preprocess: Option<String>,
    /// URL of the reference video/// URL of the reference video/// "https://github.com/OpenTalker/SadTalker/raw/main/examples/ref_video/WDA_AlexandriaOcasioCortez_000.mp4"
    pub reference_pose_video_url: String,
    /// URL of the source image/// URL of the source image/// "https://storage.googleapis.com/falserverless/model_tests/sadtalker/anime_girl.png"
    pub source_image_url: String,
    /// Whether to use still mode. Fewer head motion, works with preprocess `full`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub still_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AMTInterpolationInput {
    /// Output frames per second
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_fps: Option<i64>,
    /// Number of recursive interpolation passes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive_interpolation_passes: Option<i64>,
    /// URL of the video to be processed/// URL of the video to be processed/// "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/diffusers/animatediff-vid2vid-input-2.gif"
    pub video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OCRBoundingBoxOutputWithLabels {
    /// Processed image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Option<Image>>,
    /// Results from the model
    pub results: OCRBoundingBox,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BlurMaskOutput {
    /// The mask/// The mask/// {"content_type":"image/png","height":700,"url":"https://storage.googleapis.com/falserverless/model_tests/workflow_utils/blur_mask_output.png","width":610}
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToImageControlNetUnionInput {
    /// The URL of the control image./// The URL of the control image./// "https://fal-cdn.batuhan-941.workers.dev/files/rabbit/MiN_j3St9B8esJleCZKMU.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub canny_image_url: Option<String>,
    /// Whether to preprocess the canny image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canny_preprocess: Option<bool>,
    /// The scale of the controlnet conditioning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlnet_conditioning_scale: Option<f64>,
    /// The URL of the control image./// The URL of the control image./// "https://fal-cdn.batuhan-941.workers.dev/files/rabbit/MiN_j3St9B8esJleCZKMU.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_image_url: Option<String>,
    /// Whether to preprocess the depth image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_preprocess: Option<bool>,
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image. Leave it none to automatically infer from the control image./// The size of the generated image. Leave it none to automatically infer from the control image./// null

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"
    /// "ugly, deformed"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The URL of the control image./// The URL of the control image./// "https://fal-cdn.batuhan-941.workers.dev/files/rabbit/MiN_j3St9B8esJleCZKMU.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_image_url: Option<String>,
    /// Whether to preprocess the normal image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_preprocess: Option<bool>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The URL of the control image./// The URL of the control image./// "https://fal-cdn.batuhan-941.workers.dev/files/rabbit/MiN_j3St9B8esJleCZKMU.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub openpose_image_url: Option<String>,
    /// Whether to preprocess the openpose image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openpose_preprocess: Option<bool>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "Ice fortress, aurora skies, polar wildlife, twilight"
    pub prompt: String,
    /// An id bound to a request, can be used with response to identify the request
    /// itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The URL of the control image./// The URL of the control image./// "https://fal-cdn.batuhan-941.workers.dev/files/rabbit/MiN_j3St9B8esJleCZKMU.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_image_url: Option<String>,
    /// Whether to preprocess the segmentation image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_preprocess: Option<bool>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
    /// The URL of the control image./// The URL of the control image./// "https://fal-cdn.batuhan-941.workers.dev/files/rabbit/MiN_j3St9B8esJleCZKMU.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub teed_image_url: Option<String>,
    /// Whether to preprocess the teed image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teed_preprocess: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MarigoldDepthMapInput {
    /// Number of predictions to average over. Defaults to `10`. The higher the number, the more accurate the result, but the slower the inference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensemble_size: Option<i64>,
    /// Input image url./// Input image url./// "https://storage.googleapis.com/falserverless/model_tests/remove_background/elephant.jpg"
    pub image_url: String,
    /// Number of denoising steps. Defaults to `10`. The higher the number, the more accurate the result, but the slower the inference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// Maximum processing resolution. Defaults `0` which means it uses the size of the input image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_res: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToImageOutput {
    pub images: Vec<File>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JanusInput {
    /// Classifier Free Guidance scale - how closely to follow the prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfg_weight: Option<f64>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// Number of images to generate in parallel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "beautiful girl, inside a house"
    pub prompt: String,
    /// Random seed for reproducible generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<SeedProperty>,
    /// Controls randomness in the generation. Higher values make output more random.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct T2VDirectorOutput {
    /// The generated video/// The generated video/// {"url":"https://fal.media/files/panda/4Et1qL4cbedh-OACEw7OF_output.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Recraft20BTextToImageInput {
    /// An array of preferable colors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<Option<RGBColor>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    pub prompt: String,
    /// The style of the generated images. Vector images cost 2X as much.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    /// The ID of the custom style reference (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RFInversionInput {
    /// Base shift for the scheduled timesteps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_shift: Option<f64>,
    /// The LoRAs to use for the image generation which use a control image. You can use any number of LoRAs
    /// and they will be merged together to generate the final image./// The LoRAs to use for the image generation which use a control image. You can use any number of LoRAs
    /// and they will be merged together to generate the final image.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_loras: Option<Vec<Option<ControlLoraWeight>>>,
    /// The controller guidance (gamma) used in the creation of structured noise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_guidance_forward: Option<f64>,
    /// The controller guidance (eta) used in the denoising process.Using values closer to 1 will result in an image closer to input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_guidance_reverse: Option<f64>,
    /// The controlnet unions to use for the image generation. Only one controlnet is supported at the moment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlnet_unions: Option<Vec<Option<ControlNetUnion>>>,
    /// The controlnets to use for the image generation. Only one controlnet is supported at the moment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlnets: Option<Vec<Option<ControlNet>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// URL of image to be edited/// URL of image to be edited/// "https://storage.googleapis.com/falserverless/flux-general-tests/anime_style.png"
    pub image_url: String,
    /// The LoRAs to use for the image generation. You can use any number of LoRAs
    /// and they will be merged together to generate the final image./// The LoRAs to use for the image generation. You can use any number of LoRAs
    /// and they will be merged together to generate the final image.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// Max shift for the scheduled timesteps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_shift: Option<f64>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to edit the image with/// The prompt to edit the image with/// "Wearing glasses"
    pub prompt: String,
    /// The percentage of the total timesteps when the reference guidance is to be ended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_end: Option<f64>,
    /// URL of Image for Reference-Only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_image_url: Option<String>,
    /// The percentage of the total timesteps when the reference guidance is to bestarted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_start: Option<f64>,
    /// Strength of reference_only generation. Only used if a reference image is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_strength: Option<f64>,
    /// Timestep to stop guidance during reverse process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_guidance_end: Option<i64>,
    /// Scheduler for applying reverse guidance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_guidance_schedule: Option<String>,
    /// Timestep to start guidance during reverse process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_guidance_start: Option<i64>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
    /// Specifies whether beta sigmas ought to be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_beta_schedule: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BaseInput {
    /// The target FPS of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_fps: Option<i64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related video to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The LoRAs to use for the image generation. We currently support one lora./// The LoRAs to use for the image generation. We currently support one lora.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The negative prompt to generate video from/// The negative prompt to generate video from/// "Distorted, discontinuous, Ugly, blurry, low resolution, motionless, static, disfigured, disconnected limbs, Ugly faces, incomplete arms"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to generate the video from./// The prompt to generate the video from./// "A garden comes to life as a kaleidoscope of butterflies flutters amidst the blossoms, their delicate wings casting shadows on the petals below. In the background, a grand fountain cascades water with a gentle splendor, its rhythmic sound providing a soothing backdrop. Beneath the cool shade of a mature tree, a solitary wooden chair invites solitude and reflection, its smooth surface worn by the touch of countless visitors seeking a moment of tranquility in nature's embrace."
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same video every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Use RIFE for video interpolation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_rife: Option<bool>,
    /// The size of the generated video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_size: Option<VideoSizeProperty>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WanT2VRequest {
    /// Aspect ratio of the generated video (16:9 or 9:16).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// Whether to enable prompt expansion./// Whether to enable prompt expansion./// false

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_prompt_expansion: Option<bool>,
    /// If set to true, the safety checker will be enabled./// If set to true, the safety checker will be enabled./// true

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// Number of inference steps for sampling. Higher values give better quality but take longer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_steps: Option<i64>,
    /// Number of inference steps for sampling. Higher values give better quality but take longer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The text prompt to guide video generation./// The text prompt to guide video generation./// "A stylish woman walks down a Tokyo street filled with warm glowing neon and animated city signage. She wears a black leather jacket, a long red dress, and black boots, and carries a black purse."
    pub prompt: String,
    /// Resolution of the generated video (480p,580p, or 720p).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// Random seed for reproducibility. If None, a random seed is chosen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProFillInput {
    /// The image URL to generate an image from. Needs to match the dimensions of the mask./// The image URL to generate an image from. Needs to match the dimensions of the mask./// "https://storage.googleapis.com/falserverless/flux-lora/example-images/knight.jpeg"
    pub image_url: String,
    /// The mask URL to inpaint the image. Needs to match the dimensions of the input image./// The mask URL to inpaint the image. Needs to match the dimensions of the input image./// "https://storage.googleapis.com/falserverless/flux-lora/example-images/mask_knight.jpeg"
    pub mask_url: String,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to fill the masked part of the image./// The prompt to fill the masked part of the image./// "A knight in shining armour holding a greatshield with \"FAL\" on it"
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_tolerance: Option<String>,
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
pub struct IPAdapter {
    /// The value to set the image projection shortcut to. For FaceID plus V1 models,
    /// this should be set to False. For FaceID plus V2 models, this should be set to True.
    /// Default is True.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_projection_shortcut: Option<bool>,
    /// URL or the path to the InsightFace model weights.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_face_model_path: Option<String>,
    /// URL of the image to be used as the IP adapter.
    pub ip_adapter_image_url: IpAdapterImageUrlProperty,
    /// The mask to use for the IP adapter. When using a mask, the ip-adapter image size and the mask size must be the same
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_adapter_mask_url: Option<String>,
    /// Subfolder in the model directory where the IP adapter weights are stored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_subfolder: Option<String>,
    /// URL or the path to the IP adapter weights.
    pub path: String,
    /// The scale of the IP adapter weight. This is used to scale the IP adapter weight
    /// before merging it with the base model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    /// The scale of the IP adapter weight. This is used to scale the IP adapter weight
    /// before merging it with the base model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_json: Option<HashMap<String, serde_json::Value>>,
    /// The factor to apply to the unconditional noising of the IP adapter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unconditional_noising_factor: Option<f64>,
    /// Name of the weight file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Recraft20BTextToImageOutput {
    pub images: Vec<File>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Track {
    /// Unique identifier for the track
    pub id: String,
    /// List of keyframes that make up this track
    pub keyframes: Vec<Keyframe>,
    /// Type of track ('video' or 'audio')
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AudioTrack {
    /// Audio bitrate in bits per second
    pub bitrate: i64,
    /// Number of audio channels
    pub channels: i64,
    /// Audio codec used (e.g., 'aac', 'mp3')
    pub codec: String,
    /// Audio sample rate in Hz
    pub sample_rate: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageToImageLCMInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The rescale factor for the CFG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_rescale: Option<f64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://fal-cdn.batuhan-941.workers.dev/files/tiger/IExuP-WICqaIesLZAZPur.jpeg"
    pub image_url: String,
    /// The name of the model to use./// The name of the model to use./// "stabilityai/stable-diffusion-xl-base-1.0"
    /// "runwayml/stable-diffusion-v1-5"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "an island near sea, with seagulls, moon shining over the sea, light house, boats int he background, fish flying over the sea"
    pub prompt: String,
    /// An id bound to a request, can be used with response to identify the request
    /// itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProductShotInput {
    /// Whether to use the fast model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fast: Option<bool>,
    /// The URL of the product shot to be placed in a lifestyle shot. If both image_url and image_file are provided, image_url will be used. Accepted formats are jpeg, jpg, png, webp. Maximum file size 12MB./// The URL of the product shot to be placed in a lifestyle shot. If both image_url and image_file are provided, image_url will be used. Accepted formats are jpeg, jpg, png, webp. Maximum file size 12MB./// "https://storage.googleapis.com/falserverless/bria/bria_product_fg.jpg"
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
    /// The URL of the reference image to be used for generating the new scene or background for the product shot. Use "" to leave empty.Either ref_image_url or scene_description has to be provided but not both. If both ref_image_url and ref_image_file are provided, ref_image_url will be used. Accepted formats are jpeg, jpg, png, webp./// The URL of the reference image to be used for generating the new scene or background for the product shot. Use "" to leave empty.Either ref_image_url or scene_description has to be provided but not both. If both ref_image_url and ref_image_file are provided, ref_image_url will be used. Accepted formats are jpeg, jpg, png, webp./// "https://storage.googleapis.com/falserverless/bria/bria_product_bg.jpg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_image_url: Option<String>,
    /// Text description of the new scene or background for the provided product shot. Bria currently supports prompts in English only, excluding special characters./// Text description of the new scene or background for the provided product shot. Bria currently supports prompts in English only, excluding special characters./// "on a rock, next to the ocean, dark theme"

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
pub struct IpAdapterFaceIdOutput {
    /// The generated image file info.
    pub image: Image,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "A blackhole in space."
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_tolerance: Option<String>,
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
pub struct FastTextToVideoRequest {
    /// The aspect ratio of the generated video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// Negative prompt to be used for the generation/// Negative prompt to be used for the generation/// "blurry, low quality, low resolution, pixelated, noisy, grainy, out of focus, poorly lit, poorly exposed, poorly composed, poorly framed, poorly cropped, poorly color corrected, poorly color graded"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    pub prompt: String,
    /// The resolution of the generated video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same video every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The style of the generated video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SkyreelsI2VRequest {
    /// Aspect ratio of the output video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// Guidance scale for generation (between 1.0 and 20.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// URL of the image input./// URL of the image input./// "https://fal.media/files/panda/TuXlMwArpQcdYNCLAEM8K.webp"
    pub image_url: String,
    /// Negative prompt to guide generation away from certain attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Number of denoising steps (between 1 and 50). Higher values give better quality but take longer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to generate the video from./// The prompt to generate the video from./// "A stylish woman walks down a Tokyo street filled with warm glowing neon and animated city signage. She wears a black leather jacket, a long red dress, and black boots, and carries a black purse."
    pub prompt: String,
    /// Random seed for generation. If not provided, a random seed will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FaceToStickerInput {
    /// If set to false, the safety checker will be disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// URL of the video./// URL of the video./// "https://storage.googleapis.com/falserverless/model_tests/face_to_sticker/elon.jpg"
    pub image_url: String,
    /// The strength of the instant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_id_strength: Option<f64>,
    /// The amount of noise to add to the IP adapter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_adapter_noise: Option<f64>,
    /// The weight of the IP adapter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_adapter_weight: Option<f64>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Increasing the amount of steps tells Stable Diffusion that it should take more steps
    /// to generate your final result which can increase the amount of detail in your image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "a person"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Whether to upscale the image 2x.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upscale: Option<bool>,
    /// The number of steps to use for upscaling. Only used if `upscale` is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upscale_steps: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Input {
    /// The URL of the audio to generate the lip sync for./// The URL of the audio to generate the lip sync for./// "https://fal.media/files/lion/vyFWygmZsIZlUO4s0nr2n.wav"
    pub audio_url: String,
    /// Guidance scale for the model inference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// Video loop mode when audio is longer than video. Options: pingpong, loop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_mode: Option<LoopModeProperty>,
    /// Random seed for generation. If None, a random seed will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<SeedProperty>,
    /// The URL of the video to generate the lip sync for./// The URL of the video to generate the lip sync for./// "https://fal.media/files/koala/8teUPbRRMtAUTORDvqy0l.mp4"
    pub video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InpaintingInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
    pub image_url: String,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The URL of the mask to use for inpainting./// The URL of the mask to use for inpainting./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
    pub mask_url: String,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "a tiger sitting on a park bench"
    pub prompt: String,
    /// An id bound to a request, can be used with response to identify the request
    /// itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LCMInput {
    /// If set to true, the inpainting pipeline will use controlnet inpainting.
    /// Only effective for inpainting pipelines.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlnet_inpaint: Option<bool>,
    /// If set to true, the resulting image will be checked whether it includes any
    /// potentially unsafe content. If it does, it will be replaced with a black
    /// image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checks: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image. You can choose between some presets or
    /// custom height and width that **must be multiples of 8**.
    ///
    /// If not provided:
    /// - For text-to-image generations, the default size is 512x512.
    /// - For image-to-image generations, the default size is the same as the input image.
    /// - For inpainting generations, the default size is the same as the input image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The base image to use for guiding the image generation on image-to-image
    /// generations. If the either width or height of the image is larger than 1024
    /// pixels, the image will be resized to 1024 pixels while keeping the aspect ratio./// The base image to use for guiding the image generation on image-to-image
    /// generations. If the either width or height of the image is larger than 1024
    /// pixels, the image will be resized to 1024 pixels while keeping the aspect ratio./// "https://storage.googleapis.com/falserverless/model_tests/lcm/inpaint_image.png"
    /// "https://storage.googleapis.com/falserverless/model_tests/lcm/beach.png"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// If set to true, the inpainting pipeline will only inpaint the provided mask
    /// area. Only effective for inpainting pipelines.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inpaint_mask_only: Option<bool>,
    /// The scale of the lora server to use for image generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lora_scale: Option<f64>,
    /// The url of the lora server to use for image generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lora_url: Option<String>,
    /// The mask to use for guiding the image generation on image
    /// inpainting. The model will focus on the mask area and try to fill it with
    /// the most relevant content.
    ///
    /// The mask must be a black and white image where the white area is the area
    /// that needs to be filled and the black area is the area that should be
    /// ignored.
    ///
    /// The mask must have the same dimensions as the image passed as `image_url`./// The mask to use for guiding the image generation on image
    /// inpainting. The model will focus on the mask area and try to fill it with
    /// the most relevant content.
    ///
    /// The mask must be a black and white image where the white area is the area
    /// that needs to be filled and the black area is the area that should be
    /// ignored.
    ///
    /// The mask must have the same dimensions as the image passed as `image_url`./// "https://storage.googleapis.com/falserverless/model_tests/lcm/inpaint_mask.png"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_url: Option<String>,
    /// The model to use for generating the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"
    /// "ugly, deformed"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate. The function will return a list of images
    /// with the same prompt and negative prompt but different seeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to use for generating the image. The more steps
    /// the better the image will be but it will also take longer to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "a black cat with glowing eyes, cute, adorable, disney, pixar, highly detailed, 8k"
    /// "an island near sea, with seagulls, moon shining over the sea, light house, boats int he background, fish flying over the sea"
    pub prompt: String,
    /// An id bound to a request, can be used with response to identify the request
    /// itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// 42

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The strength of the image that is passed as `image_url`. The strength
    /// determines how much the generated image will be similar to the image passed as
    /// `image_url`. The higher the strength the more model gets "creative" and
    /// generates an image that's different from the initial image. A strength of 1.0
    /// means that the initial image is more or less ignored and the model will try to
    /// generate an image that's as close as possible to the prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VTONInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// Url to the garment image./// Url to the garment image./// "https://storage.googleapis.com/falserverless/model_tests/leffa/tshirt_image.jpg"
    pub garment_image_url: String,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your input when generating the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// Url for the human image./// Url for the human image./// "https://storage.googleapis.com/falserverless/model_tests/leffa/person_image.jpg"
    pub human_image_url: String,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The same seed and the same input given to the same version of the model
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
pub struct DiarizationSegment {
    /// Speaker ID of the segment
    pub speaker: String,
    /// Start and end timestamp of the segment
    pub timestamp: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Region {
    /// X-coordinate of the top-left corner
    pub x1: i64,
    /// X-coordinate of the bottom-right corner
    pub x2: i64,
    /// Y-coordinate of the top-left corner
    pub y1: i64,
    /// Y-coordinate of the bottom-right corner
    pub y2: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DifferentialDiffusionInput {
    /// Base shift for the scheduled timesteps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_shift: Option<f64>,
    /// URL of change map./// URL of change map./// "https://fal.media/files/zebra/Wh4IYAiAAcVbuZ8M9ZMSn.jpeg"
    pub change_map_image_url: String,
    /// The LoRAs to use for the image generation which use a control image. You can use any number of LoRAs
    /// and they will be merged together to generate the final image./// The LoRAs to use for the image generation which use a control image. You can use any number of LoRAs
    /// and they will be merged together to generate the final image.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_loras: Option<Vec<Option<ControlLoraWeight>>>,
    /// The controlnet unions to use for the image generation. Only one controlnet is supported at the moment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlnet_unions: Option<Vec<Option<ControlNetUnion>>>,
    /// The controlnets to use for the image generation. Only one controlnet is supported at the moment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlnets: Option<Vec<Option<ControlNet>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// URL of image to use as initial image./// URL of image to use as initial image./// "https://fal.media/files/koala/h6a7KK2Ie_inuGbdartoX.jpeg"
    pub image_url: String,
    /// IP-Adapter to use for image generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_adapters: Option<Vec<Option<IPAdapter>>>,
    /// The LoRAs to use for the image generation. You can use any number of LoRAs
    /// and they will be merged together to generate the final image./// The LoRAs to use for the image generation. You can use any number of LoRAs
    /// and they will be merged together to generate the final image.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// Max shift for the scheduled timesteps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_shift: Option<f64>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Tree of life under the sea, ethereal, glittering, lens flares, cinematic lighting, artwork by Anna Dittmann & Carne Griffiths, 8k, unreal engine 5, hightly detailed, intricate detailed."
    pub prompt: String,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_cfg_scale: Option<f64>,
    /// The percentage of the total timesteps when the reference guidance is to be ended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_end: Option<f64>,
    /// URL of Image for Reference-Only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_image_url: Option<String>,
    /// The percentage of the total timesteps when the reference guidance is to bestarted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_start: Option<f64>,
    /// Strength of reference_only generation. Only used if a reference image is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_strength: Option<f64>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The strength to use for differential diffusion. 1.0 is completely remakes the image while 0.0 preserves the original.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
    /// Specifies whether beta sigmas ought to be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_beta_schedule: Option<bool>,
    /// Uses classical CFG as in SD1.5, SDXL, etc. Increases generation times and price when set to be true.
    /// If using XLabs IP-Adapter v1, this will be turned on!.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_real_cfg: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Keyframe {
    /// The duration in milliseconds of this keyframe
    pub duration: f64,
    /// The timestamp in milliseconds where this keyframe starts
    pub timestamp: f64,
    /// The URL where this keyframe's media file can be accessed
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DepthInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// URL of image to use for depth input/// URL of image to use for depth input/// "https://fal.media/files/penguin/vt-SeIOweN7_oYBsvGO6t.png"
    pub image_url: String,
    /// The LoRAs to use for the image generation. You can use any number of LoRAs
    /// and they will be merged together to generate the final image./// The LoRAs to use for the image generation. You can use any number of LoRAs
    /// and they will be merged together to generate the final image.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Black hole in space, orange accretion disc"
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
pub struct TextToImagePlaygroundv25Input {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The rescale factor for the CFG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_rescale: Option<f64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"
    /// "ugly, deformed"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "Masterpiece (wide angle shot) , Easterbunny crafting an incantation, (creating a little colorful magic egg in a nest:1.6), standing on an old carved table in a colorful factory laboratory. fantastic view"
    pub prompt: String,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AnimateDiffV2VTurboOutput {
    /// Seed used for generating the video.
    pub seed: i64,
    pub timings: Timings,
    /// Generated video file./// Generated video file./// {"content_type":"video/mp4","url":"https://storage.googleapis.com/falserverless/model_tests/animatediff_v2v/rocket-output.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Ray2T2VOutput {
    /// The generated video/// The generated video/// {"url":"https://v3.fal.media/files/penguin/Om3xjcOwiSCJwrXs7DUi__output.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BoundingBox {
    /// Height of the bounding box
    pub h: f64,
    /// Label of the bounding box
    pub label: String,
    /// Width of the bounding box
    pub w: f64,
    /// X-coordinate of the top-left corner
    pub x: f64,
    /// Y-coordinate of the top-left corner
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RegexReplaceInput {
    /// Pattern to replace/// Pattern to replace/// "World"
    pub pattern: String,
    /// Replacement text/// Replacement text/// "Universe"
    pub replace: String,
    /// Input text/// Input text/// "Hello, World!"
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MandarinRequest {
    pub prompt: String,
    /// Voice ID for the desired voice./// Voice ID for the desired voice./// "zf_xiaobei"
    pub voice: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageToVideoOutput {
    /// The generated video/// The generated video/// {"url":"https://v3.fal.media/files/zebra/uNu-1qkbNt8be8iHA1hiB_output.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Output {
    pub images: Vec<File>,
    /// Seed used for generation/// Seed used for generation/// 42
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Era3DOutput {
    /// Images with background removed
    pub images: Vec<Image>,
    /// Normal images with background removed
    pub normal_images: Vec<Image>,
    /// Seed used for random number generation
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InpaintingPlaygroundv25Input {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The rescale factor for the CFG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_rescale: Option<f64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
    pub image_url: String,
    /// The URL of the mask to use for inpainting./// The URL of the mask to use for inpainting./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
    pub mask_url: String,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "a tiger sitting on a park bench"
    pub prompt: String,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProCannyControlInput {
    /// The control image URL to generate the Canny edge map from./// The control image URL to generate the Canny edge map from./// "https://fal.media/files/kangaroo/eNSkRdVFzNvDkrrMjxFA3.png"
    pub control_image_url: String,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "A pink owl."
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_tolerance: Option<String>,
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
pub struct DWPoseInput {
    /// URL of the image to be processed/// URL of the image to be processed/// "https://github.com/badayvedat/sane-controlnet-aux/blob/main/tests/data/pose_sample.jpg?raw=true"
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageToVideoRequest {
    /// The aspect ratio of the generated video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// The duration of the generated video in seconds. 8s videos cost double. 1080p videos are limited to 5 seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// URL of the image to use as the first frame/// URL of the image to use as the first frame/// "https://fal.media/files/elephant/8kkhB12hEZI2kkbU8pZPA_test.jpeg"
    pub image_url: String,
    /// Negative prompt to be used for the generation/// Negative prompt to be used for the generation/// "blurry, low quality, low resolution, pixelated, noisy, grainy, out of focus, poorly lit, poorly exposed, poorly composed, poorly framed, poorly cropped, poorly color corrected, poorly color graded"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    pub prompt: String,
    /// The resolution of the generated video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same video every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The style of the generated video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubjectReferenceOutput {
    /// The generated video/// The generated video/// {"url":"https://fal.media/files/rabbit/pONKqOnY7z6GlF6oDESvR_output.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FillInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// Use an image fill input to fill in particular images into the masked area.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_image: Option<Option<ImageFillInput>>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// URL of image to use for fill operation/// URL of image to use for fill operation/// "https://storage.googleapis.com/falserverless/flux-lora/example-images/knight.jpeg"
    pub image_url: String,
    /// The LoRAs to use for the image generation. You can use any number of LoRAs
    /// and they will be merged together to generate the final image./// The LoRAs to use for the image generation. You can use any number of LoRAs
    /// and they will be merged together to generate the final image.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The mask to area to Inpaint in./// The mask to area to Inpaint in./// "https://storage.googleapis.com/falserverless/flux-lora/example-images/mask_knight.jpeg"
    pub mask_url: String,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// Specifies whether to paste-back the original image onto to the non-inpainted areas of the output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paste_back: Option<bool>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "A knight in shining armour holding a greatshield with 'FAL' on it"
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
pub struct ZonosInput {
    /// The content generated using cloned voice./// The content generated using cloned voice./// "Fal is the fastest solution for your image generation."
    pub prompt: String,
    /// The reference audio./// The reference audio./// "https://storage.googleapis.com/falserverless/model_tests/zonos/demo_voice_zonos.wav"
    pub reference_audio_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToImageHyperInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<String>,
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
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
pub struct InpaintingControlNetUnionInput {
    /// The URL of the control image./// The URL of the control image./// "https://fal-cdn.batuhan-941.workers.dev/files/rabbit/MiN_j3St9B8esJleCZKMU.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub canny_image_url: Option<String>,
    /// Whether to preprocess the canny image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canny_preprocess: Option<bool>,
    /// The scale of the controlnet conditioning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlnet_conditioning_scale: Option<f64>,
    /// The URL of the control image./// The URL of the control image./// "https://fal-cdn.batuhan-941.workers.dev/files/rabbit/MiN_j3St9B8esJleCZKMU.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_image_url: Option<String>,
    /// Whether to preprocess the depth image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_preprocess: Option<bool>,
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image. Leave it none to automatically infer from the control image./// The size of the generated image. Leave it none to automatically infer from the control image./// null

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
    pub image_url: String,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The URL of the mask to use for inpainting./// The URL of the mask to use for inpainting./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
    pub mask_url: String,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The URL of the control image./// The URL of the control image./// "https://fal-cdn.batuhan-941.workers.dev/files/rabbit/MiN_j3St9B8esJleCZKMU.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_image_url: Option<String>,
    /// Whether to preprocess the normal image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_preprocess: Option<bool>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The URL of the control image./// The URL of the control image./// "https://fal-cdn.batuhan-941.workers.dev/files/rabbit/MiN_j3St9B8esJleCZKMU.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub openpose_image_url: Option<String>,
    /// Whether to preprocess the openpose image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openpose_preprocess: Option<bool>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "Ice fortress, aurora skies, polar wildlife, twilight"
    pub prompt: String,
    /// An id bound to a request, can be used with response to identify the request
    /// itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The URL of the control image./// The URL of the control image./// "https://fal-cdn.batuhan-941.workers.dev/files/rabbit/MiN_j3St9B8esJleCZKMU.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_image_url: Option<String>,
    /// Whether to preprocess the segmentation image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_preprocess: Option<bool>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
    /// The URL of the control image./// The URL of the control image./// "https://fal-cdn.batuhan-941.workers.dev/files/rabbit/MiN_j3St9B8esJleCZKMU.jpeg"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub teed_image_url: Option<String>,
    /// Whether to preprocess the teed image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teed_preprocess: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToImageFooocusInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, a smaller model will try to refine the output after it was processed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_refiner: Option<bool>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The rescale factor for the CFG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_rescale: Option<f64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"
    /// "ugly, deformed"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "Self-portrait oil painting, a beautiful cyborg with golden hair, 8k."
    pub prompt: String,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BrEngOutput {
    /// The generated music/// The generated music/// {"url":"https://fal.media/files/kangaroo/4wpA60Kum6UjOVBKJoNyL_tmpxfrkn95k.wav"}
    pub audio: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WhisperOutput {
    /// Timestamp chunks of the audio file
    pub chunks: Vec<WhisperChunk>,
    /// Transcription of the audio file
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VideoConditioningInput {
    /// Frame number of the video from which the conditioning starts. Must be a multiple of 8.
    pub start_frame_num: i64,
    /// URL of video to be extended
    pub video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageToImageSD15Input {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://fal-cdn.batuhan-941.workers.dev/files/tiger/IExuP-WICqaIesLZAZPur.jpeg"
    pub image_url: String,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "an island near sea, with seagulls, moon shining over the sea, light house, boats int he background, fish flying over the sea"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubjectReferenceRequest {
    pub prompt: String,
    /// Whether to use the model's prompt optimizer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_optimizer: Option<bool>,
    /// URL of the subject reference image to use for consistent subject appearance/// URL of the subject reference image to use for consistent subject appearance/// "https://fal.media/files/tiger/s2xnjhLpjM6L8ISxlDCAw.png"
    pub subject_reference_image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExtendVideoInput {
    /// Aspect ratio of the generated video (16:9 or 9:16).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// Whether to expand the prompt using the model's own capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// Negative prompt for generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Number of inference steps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// Text prompt to guide generation/// Text prompt to guide generation/// "Woman walking on a street in Tokyo"
    pub prompt: String,
    /// Resolution of the generated video (480p or 720p).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// Random seed for generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Video to be extended./// Video to be extended./// {"start_frame_num":24,"video_url":"https://storage.googleapis.com/falserverless/web-examples/wan/t2v.mp4"}
    pub video: VideoConditioningInput,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AnimateDiffT2VTurboInput {
    /// Number of frames per second to extract from the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<i64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The motions to apply to the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motions: Option<Vec<Option<String>>>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of frames to generate for the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_frames: Option<i64>,
    /// The number of inference steps to perform. 4-12 is recommended for turbo mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the video. Be as descriptive as possible for best results./// The prompt to use for generating the video. Be as descriptive as possible for best results./// "masterpiece, best quality, 1girl, solo, cherry blossoms, hanami, pink flower, white flower, spring season, wisteria, petals, flower, plum blossoms, outdoors, falling petals, white hair, black eyes"
    /// "panda playing a guitar, on a boat, in the ocean, high quality, high quality, ultra HD, realistic"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The size of the video to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_size: Option<VideoSizeProperty>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AnimatediffLCMInput {
    /// The type of controlnet to use for generating the video. The controlnet determines how the video will be animated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlnet_type: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<i64>,
    /// The URL of the first keyframe to use for the generation./// The URL of the first keyframe to use for the generation./// "https://storage.googleapis.com/falserverless/scribble2/scribble_2_1.png"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyframe_0_image_url: Option<String>,
    /// The frame index of the first keyframe to use for the generation./// The frame index of the first keyframe to use for the generation./// 0

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyframe_0_index: Option<i64>,
    /// The URL of the second keyframe to use for the generation./// The URL of the second keyframe to use for the generation./// "https://storage.googleapis.com/falserverless/scribble2/scribble_2_2.png"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyframe_1_image_url: Option<String>,
    /// The frame index of the second keyframe to use for the generation./// The frame index of the second keyframe to use for the generation./// 8

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyframe_1_index: Option<i64>,
    /// The URL of the third keyframe to use for the generation./// The URL of the third keyframe to use for the generation./// "https://storage.googleapis.com/falserverless/scribble2/scribble_2_3.png"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyframe_2_image_url: Option<String>,
    /// The frame index of the third keyframe to use for the generation./// The frame index of the third keyframe to use for the generation./// 15

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyframe_2_index: Option<i64>,
    /// The negative prompt to use. Use it to specify what you don't want./// The negative prompt to use. Use it to specify what you don't want./// "blurry, low resolution, bad, ugly, low quality, pixelated, interpolated, compression artifacts, noisey, grainy"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Increasing the amount of steps tells Stable Diffusion that it should take more steps to generate your final result which can increase the amount of detail in your image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "Drone footage, futuristic city at night, synthwave, vaporware, neon lights, highly detailed, masterpeice, high quality"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable
    /// Diffusion will output the same image every time./// The same seed and the same prompt given to the same version of Stable
    /// Diffusion will output the same image every time./// 42

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LightningModelsTextToImageInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The Lightning model to use./// The Lightning model to use./// "Lykon/dreamshaper-xl-lightning"
    /// "SG161222/RealVisXL_V4.0_Lightning"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The negative prompt to use. Use it to address details that you don't want in the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "A hyperdetailed photograph of a Cat dressed as a mafia boss holding a fish walking down a Japanese fish market with an angry face, 8k resolution, best quality, beautiful photograph, dynamic lighting,"
    pub prompt: String,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// Scheduler / sampler to use for the image denoising process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
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
pub struct InsertTextInput {
    /// Template to insert text into/// Template to insert text into/// "Hello, {}!"
    pub template: String,
    /// Input text/// Input text/// "Hello, World!"
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DocResOutput {
    /// The generated image file info./// The generated image file info./// {"content_type":"image/png","file_name":"36d3ca4791a647678b2ff01a35c87f5a.png","file_size":423052,"height":512,"url":"https://storage.googleapis.com/falserverless/docres_ckpt/Xssvg5K39QiD6mn9K5toF_f4942abeef8d4c7bbe236b59aed5e382.png","width":512}
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InvertMaskOutput {
    /// The mask/// The mask/// {"content_type":"image/png","height":700,"url":"https://storage.googleapis.com/falserverless/model_tests/workflow_utils/invert_mask_output.png","width":610}
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CompareTextInput {
    /// Text to compare against/// Text to compare against/// "Hello, World!"
    pub compare_text: String,
    /// Text to return if the input text does not match the compare text/// Text to return if the input text does not match the compare text/// "Hello, Universe!"
    pub fail_text: String,
    /// Text to return if the input text matches the compare text/// Text to return if the input text matches the compare text/// "Hello, World!"
    pub return_text: String,
    /// Input text/// Input text/// "Hello, World!"
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RemixImageInput {
    /// The aspect ratio of the generated image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// Whether to expand the prompt with MagicPrompt functionality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The image URL to remix/// The image URL to remix/// "https://fal.media/files/lion/FHOx4y4a0ef7Sgmo-sOUR_image.png"
    pub image_url: String,
    /// The prompt to remix the image with/// The prompt to remix the image with/// "An ice field in north atlantic"
    pub prompt: String,
    /// Seed for the random number generator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Strength of the input image in the remix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// The style of the generated image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct I2VDirectorOutput {
    /// The generated video/// The generated video/// {"url":"https://storage.googleapis.com/falserverless/web-examples/minimax/i2v-01.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProductShotOutput {
    /// The generated images/// The generated images/// [{"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_product_res.png"}]
    pub images: Vec<Image>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WaveformInput {
    /// URL of the audio file to analyze
    pub media_url: String,
    /// Controls how many points are sampled per second of audio. Lower values (e.g. 1-2) create a coarser waveform, higher values (e.g. 4-10) create a more detailed one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points_per_second: Option<f64>,
    /// Number of decimal places for the waveform values. Higher values provide more precision but increase payload size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<i64>,
    /// Size of the smoothing window. Higher values create a smoother waveform. Must be an odd number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoothing_window: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageSizeOutput {
    /// Image size/// Image size/// {"height":700,"width":610}
    pub image_size: HashMap<String, serde_json::Value>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to fill the masked part of the image./// The prompt to fill the masked part of the image./// "A knight in shining armour holding a greatshield with \"FAL\" on it"
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_tolerance: Option<String>,
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
pub struct NSFWImageDetectionOutput {
    /// The probability of the image being NSFW.
    pub nsfw_probability: f64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VTONOutput {
    /// Whether the image contains NSFW concepts.
    pub has_nsfw_concepts: bool,
    /// The output image./// The output image./// {"content_type":"image/jpeg","height":1024,"url":"https://fal.media/files/elephant/9NTQQNo9eyiQUSLa6cYBW.png","width":768}
    pub image: Image,
    /// The seed for the inference.
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LivePortraitOutput {
    /// The generated video file.
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxFinetuneInput {
    /// Enables/disables automatic image captioning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captioning: Option<bool>,
    /// URL to the training data
    pub data_url: String,
    /// Descriptive note to identify your fine-tune since names are UUIDs. Will be displayed in finetune_details./// Descriptive note to identify your fine-tune since names are UUIDs. Will be displayed in finetune_details./// "test-1"
    pub finetune_comment: String,
    /// Choose between 'full' for a full finetuning + post hoc extraction of the trained weights into a LoRA or 'lora' for a raw LoRA training/// Choose between 'full' for a full finetuning + post hoc extraction of the trained weights into a LoRA or 'lora' for a raw LoRA training/// "full"
    /// "lora"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub finetune_type: Option<String>,
    /// Defines training duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<i64>,
    /// Learning rate for training. Lower values may be needed for certain scenarios. Default is 1e-5 for full and 1e-4 for LoRA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learning_rate: Option<f64>,
    /// Choose between 32 and 16. A lora_rank of 16 can increase training efficiency and decrease loading times./// Choose between 32 and 16. A lora_rank of 16 can increase training efficiency and decrease loading times./// 32
    /// 16

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lora_rank: Option<i64>,
    /// Determines the finetuning approach based on your concept/// Determines the finetuning approach based on your concept/// "character"
    /// "product"
    /// "style"
    /// "general"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// The speed priority will improve training and inference speed/// The speed priority will improve training and inference speed/// "quality"
    /// "speed"
    /// "high_res_only"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// Unique word/phrase that will be used in the captions, to reference the newly introduced concepts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_word: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MLSDOutput {
    /// Image with lines detected using the MLSD detector
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FooocusUpscaleOrVaryInput {
    /// The size of the generated image. You can choose between some presets or
    /// custom height and width that **must be multiples of 8**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// If set to false, the safety checker will be disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_prompt_1: Option<ImagePrompt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_prompt_2: Option<ImagePrompt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_prompt_3: Option<ImagePrompt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_prompt_4: Option<ImagePrompt>,
    /// The LoRAs to use for the image generation. You can use up to 5 LoRAs
    /// and they will be merged together to generate the final image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// Mixing Image Prompt and Vary/Upscale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixing_image_prompt_and_vary_upscale: Option<bool>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "(worst quality, low quality, normal quality, lowres, low details, oversaturated, undersaturated, overexposed, underexposed, grayscale, bw, bad photo, bad photography, bad art:1.4), (watermark, signature, text font, username, error, logo, words, letters, digits, autograph, trademark, name:1.2), (blur, blurry, grainy), morbid, ugly, asymmetrical, mutated malformed, mutilated, poorly lit, bad shadow, draft, cropped, out of frame, cut off, censored, jpeg artifacts, out of focus, glitch, duplicate, (airbrushed, cartoon, anime, semi-realistic, cgi, render, blender, digital art, manga, amateur:1.3), (3D ,3D Game, 3D Game Scene, 3D Character:1.1), (bad hands, bad anatomy, bad body, bad face, bad teeth, bad arms, bad legs, deformities:1.3)"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Number of images to generate in one request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// You can choose Speed or Quality
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<String>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "a basket of various fruits, bokeh, realistic, masterpiece"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Refiner (SDXL or SD 1.5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refiner_model: Option<String>,
    /// Use 0.4 for SD1.5 realistic models; 0.667 for SD1.5 anime models
    /// 0.8 for XL-refiners; or any value for switching two SDXL models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refiner_switch: Option<f64>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// 176400

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The sharpness of the generated image. Use it to control how sharp the generated
    /// image should be. Higher value means image and texture are sharper.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpness: Option<f64>,
    /// The style to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<Vec<Option<String>>>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
    /// The image to upscale or vary./// The image to upscale or vary./// "https://storage.googleapis.com/falserverless/model_tests/fooocus/fruit_basket.jpeg"
    pub uov_image_url: String,
    /// The method to use for upscaling or varying.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uov_method: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LLavaInput {
    /// URL of the image to be processed/// URL of the image to be processed/// "https://llava-vl.github.io/static/images/monalisa.jpg"
    pub image_url: String,
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    /// Prompt to be used for the image/// Prompt to be used for the image/// "Do you know who drew this painting?"
    pub prompt: String,
    /// Temperature for sampling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Top P for sampling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TTSOutput {
    /// The generated audio file/// The generated audio file/// {"url":"https://v3.fal.media/files/zebra/zJL_oRY8h5RWwjoK1w7tx_output.mp3"}
    pub audio: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToImageControlNetInput {
    /// The URL of the control image./// The URL of the control image./// "https://avatars.githubusercontent.com/u/74778219"
    pub control_image_url: String,
    /// The scale of the controlnet conditioning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlnet_conditioning_scale: Option<f64>,
    /// If set to true, DeepCache will be enabled. TBD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_deep_cache: Option<bool>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image. Leave it none to automatically infer from the control image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"
    /// "ugly, deformed"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "Ice fortress, aurora skies, polar wildlife, twilight"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
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
pub struct DepthMapInput {
    /// a
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<f64>,
    /// bg_th
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg_th: Option<f64>,
    /// depth_and_normal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_and_normal: Option<bool>,
    /// Input image url./// Input image url./// "https://storage.googleapis.com/falserverless/model_tests/remove_background/elephant.jpg"
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageSize {
    /// The height of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// The width of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageOutput {
    /// The output image/// The output image/// {"content_type":"image/png","height":700,"url":"https://storage.googleapis.com/falserverless/model_tests/workflow_utils/invert_mask_output.png","width":610}
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DepthAnythingV2Output {
    /// Image with depth map
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MiniMaxTextToImageOutput {
    /// Generated images
    pub images: Vec<File>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Rodin3DInput {
    /// When generating the human-like model, this parameter control the generation result to T/A Pose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tapose: Option<bool>,
    /// Generation add-on features. Default is []. Possible values are HighPack. The HighPack option will provide 4K resolution textures instead of the default 1K, as well as models with high-poly. It will cost triple the billable units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<AddonsProperty>,
    /// An array that specifies the dimensions and scaling factor of the bounding box. Typically, this array contains 3 elements, Length(X-axis), Width(Y-axis) and Height(Z-axis).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox_condition: Option<BboxConditionProperty>,
    /// For fuse mode, One or more images are required.It will generate a model by extracting and fusing features of objects from multiple images.For concat mode, need to upload multiple multi-view images of the same object and generate the model.(You can upload multi-view images in any order, regardless of the order of view.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_mode: Option<String>,
    /// Format of the geometry file. Possible values: glb, usdz, fbx, obj, stl. Default is glb.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry_file_format: Option<String>,
    /// URL of images to use while generating the 3D model. Required for Image-to-3D mode. Optional for Text-to-3D mode./// URL of images to use while generating the 3D model. Required for Image-to-3D mode. Optional for Text-to-3D mode./// "https://storage.googleapis.com/falserverless/model_tests/video_models/robot.png"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_image_urls: Option<Vec<Option<String>>>,
    /// Material type. Possible values: PBR, Shaded. Default is PBR./// Material type. Possible values: PBR, Shaded. Default is PBR./// "Shaded"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    /// A textual prompt to guide model generation. Required for Text-to-3D mode. Optional for Image-to-3D mode./// A textual prompt to guide model generation. Required for Text-to-3D mode. Optional for Image-to-3D mode./// "A futuristic robot with sleek metallic design."

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Generation quality. Possible values: high, medium, low, extra-low. Default is medium.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
    /// Seed value for randomization, ranging from 0 to 65535. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<SeedProperty>,
    /// Tier of generation. For Rodin Sketch, set to Sketch. For Rodin Regular, set to Regular.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// Whether to export the model using hyper mode. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_hyper: Option<bool>,
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
    /// The prompt you would like to use to generate images./// The prompt you would like to use to generate images./// "A lone figure stands on the edge of a serene cliff at sunset, gazing out over a vast, mystical valley. The figure is clad in flowing robes that ripple in the gentle breeze, silhouetted against the golden and lavender hues of the sky. Below, a cascading waterfall pours into a sparkling river winding through a forest of bioluminescent trees. The scene blends the awe of nature with a touch of otherworldly wonder, inviting reflection and imagination."
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
pub struct ExtendVideoOutput {
    /// The seed used for generation.
    pub seed: i64,
    /// The generated video file./// The generated video file./// {"url":"https://storage.googleapis.com/falserverless/example_outputs/ltx-v095_extend.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextInput {
    /// The conditoning augmentation determines the amount of noise that will be
    /// added to the conditioning frame. The higher the number, the more noise
    /// there will be, and the less the video will look like the initial image.
    /// Increase it for more motion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cond_aug: Option<f64>,
    /// The motion bucket id determines the motion of the generated video. The
    /// higher the number, the more motion there will be.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_bucket_id: Option<i64>,
    /// The negative prompt to use as a starting point for the generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The prompt to use as a starting point for the generation./// The prompt to use as a starting point for the generation./// "A rocket flying that is about to take off"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The size of the generated video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_size: Option<VideoSizeProperty>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FastSVDTextInput {
    /// The conditoning augmentation determines the amount of noise that will be
    /// added to the conditioning frame. The higher the number, the more noise
    /// there will be, and the less the video will look like the initial image.
    /// Increase it for more motion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cond_aug: Option<f64>,
    /// The FPS of the generated video. The higher the number, the faster the video will
    /// play. Total video length is 25 frames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<i64>,
    /// The motion bucket id determines the motion of the generated video. The
    /// higher the number, the more motion there will be.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_bucket_id: Option<i64>,
    /// The prompt to use as a starting point for the generation./// The prompt to use as a starting point for the generation./// "A rocket flying that is about to take off"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The number of steps to run the model for. The higher the number the better
    /// the quality and longer it will take to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<i64>,
    /// The size of the generated video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_size: Option<VideoSizeProperty>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MulticonditioningVideoOutput {
    /// The seed used for generation.
    pub seed: i64,
    /// The generated video file./// The generated video file./// {"url":"https://storage.googleapis.com/falserverless/gallery/ltx-multicondition.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToSpeechRequest {
    /// Similarity boost (0-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity_boost: Option<f64>,
    /// Voice stability (0-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability: Option<f64>,
    /// Style exaggeration (0-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<f64>,
    /// The text to convert to speech/// The text to convert to speech/// "Hello! This is a test of the text to speech system, powered by ElevenLabs. How does it sound?"
    pub text: String,
    /// The voice to use for speech generation/// The voice to use for speech generation/// "Aria"
    /// "Roger"
    /// "Sarah"
    /// "Laura"
    /// "Charlie"
    /// "George"
    /// "Callum"
    /// "River"
    /// "Liam"
    /// "Charlotte"
    /// "Alice"
    /// "Matilda"
    /// "Will"
    /// "Jessica"
    /// "Eric"
    /// "Chris"
    /// "Brian"
    /// "Daniel"
    /// "Lily"
    /// "Bill"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EyeCorrectOutput {
    /// The corrected video
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProUltraTextToImageFinetunedInput {
    /// The aspect ratio of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatioProperty>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// References your specific model
    pub finetune_id: String,
    /// Controls finetune influence.
    /// Increase this value if your target concept isn't showing up strongly enough.
    /// The optimal setting depends on your finetune and prompt
    pub finetune_strength: f64,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
    pub prompt: String,
    /// Generate less processed, more natural-looking images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<bool>,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_tolerance: Option<String>,
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
pub struct FooocusImagePromptInput {
    /// The size of the generated image. You can choose between some presets or
    /// custom height and width that **must be multiples of 8**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// If set to false, the safety checker will be disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    pub image_prompt_1: ImagePrompt,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_prompt_2: Option<ImagePrompt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_prompt_3: Option<ImagePrompt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_prompt_4: Option<ImagePrompt>,
    /// Describe what you want to inpaint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inpaint_additional_prompt: Option<String>,
    /// The image to use as a reference for inpainting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inpaint_image_url: Option<String>,
    /// The mode to use for inpainting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inpaint_mode: Option<String>,
    /// The LoRAs to use for the image generation. You can use up to 5 LoRAs
    /// and they will be merged together to generate the final image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The image to use as a mask for the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_image_url: Option<String>,
    /// Mixing Image Prompt and Inpaint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixing_image_prompt_and_inpaint: Option<bool>,
    /// Mixing Image Prompt and Vary/Upscale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixing_image_prompt_and_vary_upscale: Option<bool>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "(worst quality, low quality, normal quality, lowres, low details, oversaturated, undersaturated, overexposed, underexposed, grayscale, bw, bad photo, bad photography, bad art:1.4), (watermark, signature, text font, username, error, logo, words, letters, digits, autograph, trademark, name:1.2), (blur, blurry, grainy), morbid, ugly, asymmetrical, mutated malformed, mutilated, poorly lit, bad shadow, draft, cropped, out of frame, cut off, censored, jpeg artifacts, out of focus, glitch, duplicate, (airbrushed, cartoon, anime, semi-realistic, cgi, render, blender, digital art, manga, amateur:1.3), (3D ,3D Game, 3D Game Scene, 3D Character:1.1), (bad hands, bad anatomy, bad body, bad face, bad teeth, bad arms, bad legs, deformities:1.3)"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Number of images to generate in one request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The directions to outpaint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpaint_selections: Option<Vec<Option<String>>>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// You can choose Speed or Quality
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<String>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "pikachu"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Refiner (SDXL or SD 1.5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refiner_model: Option<String>,
    /// Use 0.4 for SD1.5 realistic models; 0.667 for SD1.5 anime models
    /// 0.8 for XL-refiners; or any value for switching two SDXL models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refiner_switch: Option<f64>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// 176400

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The sharpness of the generated image. Use it to control how sharp the generated
    /// image should be. Higher value means image and texture are sharper.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpness: Option<f64>,
    /// The style to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<Vec<Option<String>>>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
    /// The image to upscale or vary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uov_image_url: Option<String>,
    /// The method to use for upscaling or varying.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uov_method: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct V1ImageToVideoRequest {
    /// The aspect ratio of the generated video frame
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// The duration of the generated video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// List of dynamic masks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_masks: Option<Vec<Option<DynamicMask>>>,
    /// URL of the image to be used for the video/// URL of the image to be used for the video/// "https://h2.inkwai.com/bs2/upload-ylab-stunt/se/ai_portal_queue_mmu_image_upscale_aiweb/3214b798-e1b4-4b00-b7af-72b5b0417420_raw_image_0.jpg"
    pub image_url: String,
    /// The prompt for the video/// The prompt for the video/// "The astronaut stood up and walked away"
    pub prompt: String,
    /// URL of the image for Static Brush Application Area (Mask image created by users using the motion brush)/// URL of the image for Static Brush Application Area (Mask image created by users using the motion brush)/// "https://h2.inkwai.com/bs2/upload-ylab-stunt/ai_portal/1732888177/cOLNrShrSO/static_mask.png"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_mask_url: Option<String>,
    /// URL of the image to be used for the end of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail_image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HindiRequest {
    pub prompt: String,
    /// Voice ID for the desired voice./// Voice ID for the desired voice./// "hf_alpha"
    pub voice: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TranscriptionWord {
    /// End time in seconds
    pub end: f64,
    /// Speaker identifier if diarization was enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_id: Option<SpeakerIdProperty>,
    /// Start time in seconds
    pub start: f64,
    /// The transcribed word or audio event
    pub text: String,
    /// Type of element (word, spacing, or audio_event)
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxPulidInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The weight of the ID loss.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_weight: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The maximum sequence length for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sequence_length: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "bad quality, worst quality, text, signature, watermark, extra limbs"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "a woman holding sign with glowing green text 'PuLID for FLUX'"
    pub prompt: String,
    /// URL of image to use for inpainting./// URL of image to use for inpainting./// "https://storage.googleapis.com/falserverless/gallery/example_inputs_liuyifei.png"
    pub reference_image_url: String,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The number of steps to start the CFG from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_step: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
    /// The weight of the CFG loss.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub true_cfg: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LCMI2IInput {
    /// If set to true, the resulting image will be checked whether it includes any
    /// potentially unsafe content. If it does, it will be replaced with a black
    /// image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checks: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The image to use as a base./// The image to use as a base./// "https://storage.googleapis.com/falserverless/model_tests/lcm/beach.png"
    pub image_url: String,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"
    /// "ugly, deformed"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate. The function will return a list of images
    /// with the same prompt and negative prompt but different seeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to use for generating the image. The more steps
    /// the better the image will be but it will also take longer to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "masterpiece, colorful, photo of a beach in hawaii, sun"
    pub prompt: String,
    /// An id bound to a request, can be used with response to identify the request
    /// itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// 42

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The strength of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SoundEffectOutput {
    /// The generated sound effect audio file in MP3 format/// The generated sound effect audio file in MP3 format/// {"url":"https://v3.fal.media/files/lion/WgnO-jy6WduosuG_Ibobx_sound_effect.mp3"}
    pub audio: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TryOnOutput {
    /// The output image./// The output image./// {"content_type":"image/png","file_name":"result.png","file_size":595094,"height":1024,"url":"https://v3.fal.media/files/panda/Hoy3zhimzVKi3F2uoGBnh_result.png","width":768}
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DiffusionEdgeInput {
    /// The text prompt you would like to convert to speech./// The text prompt you would like to convert to speech./// "https://storage.googleapis.com/falserverless/model_tests/upscale/hamburger.png"
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SwittiOutput {
    /// Whether the generated images contain NSFW concepts.
    pub has_nsfw_concepts: Vec<bool>,
    /// The generated images/// The generated images/// [{"content_type":"image/jpeg","height":1024,"url":"https://fal.media/files/lion/JpgBX7w379jHteLeeNsM5.jpeg","width":1024}]
    pub images: Vec<Image>,
    /// The prompt used for generating the image.
    pub prompt: String,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
    pub timings: Timings,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PixArtSigmaInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"
    /// "ugly, deformed"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "Photorealistic closeup video of two pirate ships battling each other as they sail inside a cup of coffee."
    /// "an astronaut sitting in a diner, eating fries, cinematic, analog film"
    /// "Pirate ship trapped in a cosmic maelstrom nebula, rendered in cosmic beach whirlpool engine, volumetric lighting, spectacular, ambient lights, light pollution, cinematic atmosphere, art nouveau style, illustration art artwork by SenseiJaye, intricate detail."
    /// "stars, water, brilliantly, gorgeous large scale scene, a little girl, in the style of dreamy realism, light gold and amber, blue and pink, brilliantly illuminated in the background."
    /// "professional portrait photo of an anthropomorphic cat wearing fancy gentleman hat and jacket walking in autumn forest."
    /// "beautiful lady, freckles, big smile, blue eyes, short ginger hair, dark makeup, wearing a floral blue vest top, soft light, dark grey background"
    /// "Spectacular Tiny World in the Transparent Jar On the Table, interior of the Great Hall, Elaborate, Carved Architecture, Anatomy, Symmetrical, Geometric and Parameteric Details, Precision Flat line Details, Pattern, Dark fantasy, Dark errie mood and ineffably mysterious mood, Technical design, Intricate Ultra Detail, Ornate Detail, Stylized and Futuristic and Biomorphic Details, Architectural Concept, Low contrast Details, Cinematic Lighting, 8k, by moebius, Fullshot, Epic, Fullshot, Octane render, Unreal ,Photorealistic, Hyperrealism"
    /// "anthropomorphic profile of the white snow owl Crystal priestess , art deco painting, pretty and expressive eyes, ornate costume, mythical, ethereal, intricate, elaborate, hyperrealism, hyper detailed, 3D, 8K, Ultra Realistic, high octane, ultra resolution, amazing detail, perfection, In frame, photorealistic, cinematic lighting, visual clarity, shading , Lumen Reflections, Super-Resolution, gigapixel, color grading, retouch, enhanced, PBR, Blender, V-ray, Procreate, zBrush, Unreal Engine 5, cinematic, volumetric, dramatic, neon lighting, wide angle lens ,no digital painting blur"
    /// "The parametric hotel lobby is a sleek and modern space with plenty of natural light. The lobby is spacious and open with a variety of seating options. The front desk is a sleek white counter with a parametric design. The walls are a light blue color with parametric patterns. The floor is a light wood color with a parametric design. There are plenty of plants and flowers throughout the space. The overall effect is a calm and relaxing space. occlusion, moody, sunset, concept art, octane rendering, 8k, highly detailed, concept art, highly detailed, beautiful scenery, cinematic, beautiful light, hyperreal, octane render, hdr, long exposure, 8K, realistic, fog, moody, fire and explosions, smoke, 50mm f2.8"
    pub prompt: String,
    /// The scheduler to use for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The style to apply to the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TrainingInput {
    /// The name of the training job (required, max 255 characters)./// The name of the training job (required, max 255 characters)./// "my voice"
    pub name: String,
    /// A list of audio URLs used for training (must be between 1 and 5 URLs)./// A list of audio URLs used for training (must be between 1 and 5 URLs)./// [{"audio_url":"https://storage.googleapis.com/falserverless/model_tests/f5-tts/en_1_ref.mp3"}]
    pub training_data: Vec<AudioInput>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ControlNetUnion {
    /// optional URL to the controlnet config.json file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_url: Option<String>,
    /// The control images and modes to use for the control net.
    pub controls: Vec<ControlNetUnionInput>,
    /// URL or the path to the control net weights.
    pub path: String,
    /// The optional variant if a Hugging Face repo key is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PiDiOutput {
    /// Image with Pidi lines detected
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateVoiceOutput {
    /// The S3 URI of the cloned voice.
    pub voice: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToVideoDirectorRequest {
    /// Text prompt for video generation. Camera movement instructions can be added using square brackets (e.g. [Pan left] or [Zoom in]). You can use up to 3 combined movements per prompt. Supported movements: Truck left/right, Pan left/right, Push in/Pull out, Pedestal up/down, Tilt up/down, Zoom in/out, Shake, Tracking shot, Static shot. For example: [Truck left, Pan right, Zoom in]. For a more detailed guide, refer https://sixth-switch-2ac.notion.site/T2V-01-Director-Model-Tutorial-with-camera-movement-1886c20a98eb80f395b8e05291ad8645/// Text prompt for video generation. Camera movement instructions can be added using square brackets (e.g. [Pan left] or [Zoom in]). You can use up to 3 combined movements per prompt. Supported movements: Truck left/right, Pan left/right, Push in/Pull out, Pedestal up/down, Tilt up/down, Zoom in/out, Shake, Tracking shot, Static shot. For example: [Truck left, Pan right, Zoom in]. For a more detailed guide, refer https://sixth-switch-2ac.notion.site/T2V-01-Director-Model-Tutorial-with-camera-movement-1886c20a98eb80f395b8e05291ad8645/// "[Push in]Close up of a tense woman looks to the left, startled by a sound, in a darkened kitchen, Pots and pans hang ominously, the window in the kitchen is open and the wind softly blows the pans and creates an ominous mood. [Shake]the woman’s shock turns to fear. Black-and-white film noir shot dimly lit, 1950s-style, with dramatic, high-contrast shadows. The overall atmosphere is reminiscent of Alfred Hitchcock’s suspenseful storytelling, evoking a looming sense of dread with stark chiaroscuro lighting and a slight film-grain texture."
    pub prompt: String,
    /// Whether to use the model's prompt optimizer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_optimizer: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ChatOutput {
    /// Error message if an error occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorProperty>,
    /// Generated output/// Generated output/// "The meaning of life is subjective and depends on individual perspectives."
    pub output: String,
    /// Whether the output is partial
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<bool>,
    /// Generated reasoning for the final answer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Polygon {
    /// Label of the polygon
    pub label: String,
    /// List of points
    pub points: Vec<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Ray2TextToVideoRequest {
    /// The aspect ratio of the generated video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// The duration of the generated video (9s costs 2x more)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Whether the video should loop (end of video is blended with the beginning)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "loop")]
    pub r#loop: Option<bool>,
    pub prompt: String,
    /// The resolution of the generated video (720p costs 2x more)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OmniZeroInput {
    /// Composition image url./// Composition image url./// "https://storage.googleapis.com/falserverless/model_tests/omni_zero/structure.jpg"
    pub composition_image_url: String,
    /// Composition strength./// Composition strength./// 1

    #[serde(skip_serializing_if = "Option::is_none")]
    pub composition_strength: Option<f64>,
    /// Depth strength./// Depth strength./// 0.5

    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_strength: Option<f64>,
    /// Face strength./// Face strength./// 1

    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_strength: Option<f64>,
    /// Guidance scale./// Guidance scale./// 5

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// Identity image url./// Identity image url./// "https://storage.googleapis.com/falserverless/model_tests/omni_zero/identity.jpg"
    pub identity_image_url: String,
    /// Identity strength./// Identity strength./// 1

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_strength: Option<f64>,
    /// Image strength./// Image strength./// 0.75

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_strength: Option<f64>,
    /// Input image url./// Input image url./// "https://storage.googleapis.com/falserverless/model_tests/omni_zero/structure.jpg"
    pub image_url: String,
    /// Negative prompt to guide the image generation./// Negative prompt to guide the image generation./// ""

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Number of images./// Number of images./// 1

    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_images: Option<i64>,
    /// Prompt to guide the image generation./// Prompt to guide the image generation./// "A woman"
    pub prompt: String,
    /// Seed./// Seed./// 42

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Style image url./// Style image url./// "https://storage.googleapis.com/falserverless/model_tests/omni_zero/style.jpg"
    pub style_image_url: String,
    /// Style strength./// Style strength./// 1

    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_strength: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IllusionDiffusionInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_guidance_end: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_guidance_start: Option<f64>,
    /// The scale of the ControlNet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlnet_conditioning_scale: Option<f64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image. You can choose between some presets or
    /// custom height and width that **must be multiples of 8**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// Input image url./// Input image url./// "https://storage.googleapis.com/falserverless/illusion-examples/pattern.png"
    /// "https://storage.googleapis.com/falserverless/illusion-examples/checkers.png"
    /// "https://storage.googleapis.com/falserverless/illusion-examples/checkers_mid.jpg"
    /// "https://storage.googleapis.com/falserverless/illusion-examples/ultra_checkers.png"
    /// "https://storage.googleapis.com/falserverless/illusion-examples/funky.jpeg"
    /// "https://storage.googleapis.com/falserverless/illusion-examples/cubes.jpeg"
    /// "https://storage.googleapis.com/falserverless/illusion-examples/turkey-flag.png"
    /// "https://storage.googleapis.com/falserverless/illusion-examples/india-flag.png"
    /// "https://storage.googleapis.com/falserverless/illusion-examples/usa-flag.png"
    pub image_url: String,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "(worst quality, poor details:1.4), lowres, (artist name, signature, watermark:1.4), bad-artist-anime, bad_prompt_version2, bad-hands-5, ng_deepnegative_v1_75t"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Increasing the amount of steps tells Stable Diffusion that it should take more steps
    /// to generate your final result which can increase the amount of detail in your image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "(masterpiece:1.4), (best quality), (detailed), Medieval village scene with busy streets and castle in the distance"
    pub prompt: String,
    /// Scheduler / sampler to use for the image denoising process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DreamshaperTextToImageInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The Dreamshaper model to use./// The Dreamshaper model to use./// "Lykon/dreamshaper-8"
    /// "Lykon/dreamshaper-xl-1-0"
    /// "Lykon/dreamshaper-xl-v2-turbo"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The negative prompt to use. Use it to address details that you don't want in the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "A hyperdetailed photograph of a Cat dressed as a mafia boss holding a fish walking down a Japanese fish market with an angry face, 8k resolution, best quality, beautiful photograph, dynamic lighting,"
    pub prompt: String,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
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
pub struct MoonDreamOutput {
    /// Response from the model
    pub output: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DDColorOutput {
    /// The generated image file info./// The generated image file info./// {"content_type":"image/png","file_name":"36d3ca4791a647678b2ff01a35c87f5a.png","file_size":423052,"height":512,"url":"https://storage.googleapis.com/falserverless/gallery/5fcaaac6d1344d998ebb9703102c6c63.png","width":512}
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToVideoInput {
    /// The aspect ratio of the generated video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// The duration of the generated video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// The text prompt describing the video you want to generate/// The text prompt describing the video you want to generate/// "The camera floats gently through rows of pastel-painted wooden beehives, buzzing honeybees gliding in and out of frame. The motion settles on the refined farmer standing at the center, his pristine white beekeeping suit gleaming in the golden afternoon light. He lifts a jar of honey, tilting it slightly to catch the light. Behind him, tall sunflowers sway rhythmically in the breeze, their petals glowing in the warm sunlight. The camera tilts upward to reveal a retro farmhouse with mint-green shutters, its walls dappled with shadows from swaying trees. Shot with a 35mm lens on Kodak Portra 400 film, the golden light creates rich textures on the farmer's gloves, marmalade jar, and weathered wood of the beehives."
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextOutput {
    /// The output text/// The output text/// "Hello, World!"
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OCRBoundingBoxSingle {
    /// Height of the bounding box
    pub h: f64,
    /// Label of the bounding box
    pub label: String,
    /// Width of the bounding box
    pub w: f64,
    /// X-coordinate of the top-left corner
    pub x: f64,
    /// Y-coordinate of the top-left corner
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchedMoondreamOutput {
    /// Filenames of the images processed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filenames: Option<Vec<Option<String>>>,
    /// List of generated outputs
    pub outputs: Vec<String>,
    /// Whether the output is partial
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<bool>,
    /// Timings for different parts of the process
    pub timings: Timings,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HunyuanT2VRequest {
    /// The aspect ratio of the video to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// If set to true, the safety checker will be enabled./// If set to true, the safety checker will be enabled./// true

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The LoRAs to use for the image generation. You can use any number of LoRAs
    /// and they will be merged together to generate the final image./// The LoRAs to use for the image generation. You can use any number of LoRAs
    /// and they will be merged together to generate the final image.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The number of frames to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_frames: Option<String>,
    /// By default, generations are done with 35 steps. Pro mode does 55 steps which results in higher quality videos but will take more time and cost 2x more billing units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pro_mode: Option<bool>,
    /// The prompt to generate the video from./// The prompt to generate the video from./// "A stylish woman walks down a Tokyo street filled with warm glowing neon and animated city signage. She wears a black leather jacket, a long red dress, and black boots, and carries a black purse."
    pub prompt: String,
    /// The resolution of the video to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// The seed to use for generating the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
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
    /// The prompt you would like to use to generate images./// The prompt you would like to use to generate images./// "A 2d illustration of a dog in a vibrant park"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The URL of the structure reference image. Use "" to leave empty. Accepted formats are jpeg, jpg, png, webp./// The URL of the structure reference image. Use "" to leave empty. Accepted formats are jpeg, jpg, png, webp./// "https://storage.googleapis.com/falserverless/bria/bria_reimagine_input.png"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure_image_url: Option<String>,
    /// The influence of the structure reference on the generated image./// The influence of the structure reference on the generated image./// 0.15

    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure_ref_influence: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CameraControl {
    /// The type of camera movement/// The type of camera movement/// "horizontal"
    pub movement_type: String,
    /// The value of the camera movement/// The value of the camera movement/// 10
    pub movement_value: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CaptionInput {
    /// Size of text in generated captions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i64>,
    /// Left-to-right alignment of the text. Can be a string ('left', 'center', 'right') or a float (0.0-1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_align: Option<LeftAlignProperty>,
    /// Number of seconds the captions should stay on screen. A higher number will also result in more text being displayed at once.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_interval: Option<f64>,
    /// Width of the text strokes in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<i64>,
    /// Top-to-bottom alignment of the text. Can be a string ('top', 'center', 'bottom') or a float (0.0-1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_align: Option<TopAlignProperty>,
    /// Colour of the text. Can be a RGB tuple, a color name, or an hexadecimal notation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_color: Option<String>,
    /// Font for generated captions. Choose one in 'Arial','Standard','Garamond', 'Times New Roman','Georgia', or pass a url to a .ttf file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_font: Option<String>,
    /// URL to the .mp4 video with audio. Only videos of size <100MB are allowed.
    pub video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DocResInputDewarp {
    /// URL of image to be used for relighting/// URL of image to be used for relighting/// "https://storage.googleapis.com/falserverless/docres_ckpt/218_in.png"
    pub image_url: String,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BGRemoveInput {
    /// Input Image to erase from/// Input Image to erase from/// "https://fal.media/files/panda/K5Rndvzmn1j-OI1VZXDVd.jpeg"
    pub image_url: String,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProTextToImageInput {
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "Extreme close-up of a single tiger eye, direct frontal view. Detailed iris and pupil. Sharp focus on eye texture and color. Natural lighting to capture authentic eye shine and depth. The word \"FLUX\" is painted over it in big, white brush strokes with visible texture."
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_tolerance: Option<String>,
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
pub struct DevImageToImageInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The URL of the image to generate an image from./// The URL of the image to generate an image from./// "https://fal.media/files/koala/Chls9L2ZnvuipUTEwlnJC.png"
    pub image_url: String,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "a cat dressed as a wizard with a background of a mystic forest."
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The strength of the initial image. Higher strength values are better for this model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "A pink owl."
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_tolerance: Option<String>,
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
    /// Generated Images/// Generated Images/// [{"content_type":"image/png","file_name":"a0d138e6820c4ad58f1fd3c758f16047.png","file_size":1064550,"height":768,"url":"https://storage.googleapis.com/falserverless/bria/bria_genfill_res.png","width":1024}]
    pub images: Vec<Image>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageFillInput {
    /// URLs of images to be filled into the masked area.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_image_url: Option<FillImageUrlProperty>,
    /// Uses the provided fill image in context with the base image to fill in more faithfully. Will increase price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_context_fill: Option<bool>,
    /// Whether to use the prompt as well in the generation, along with the redux image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_prompt: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FastImageToVideoRequest {
    /// The aspect ratio of the generated video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// URL of the image to use as the first frame/// URL of the image to use as the first frame/// "https://fal.media/files/elephant/8kkhB12hEZI2kkbU8pZPA_test.jpeg"
    pub image_url: String,
    /// Negative prompt to be used for the generation/// Negative prompt to be used for the generation/// "blurry, low quality, low resolution, pixelated, noisy, grainy, out of focus, poorly lit, poorly exposed, poorly composed, poorly framed, poorly cropped, poorly color corrected, poorly color graded"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    pub prompt: String,
    /// The resolution of the generated video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same video every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The style of the generated video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VisionInput {
    /// URL of the image to be processed/// URL of the image to be processed/// "https://fal.media/files/tiger/4Ew1xYW6oZCs6STQVC7V8_86440216d0fe42e4b826d03a2121468e.jpg"
    pub image_url: String,
    /// Name of the model to use. Premium models are charged at 3x the rate of standard models, they include: meta-llama/llama-3.2-90b-vision-instruct, openai/gpt-4o, anthropic/claude-3-5-haiku, google/gemini-pro-1.5, anthropic/claude-3.5-sonnet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Prompt to be used for the image/// Prompt to be used for the image/// "Caption this image for a text-to-image model with as much detail as possible."
    pub prompt: String,
    /// Should reasoning be the part of the final answer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<bool>,
    /// System prompt to provide context or instructions to the model/// System prompt to provide context or instructions to the model/// "Only answer the question, do not provide any additional information or add any prefix/suffix other than the answer of the original question. Don't use markdown."

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<SystemPromptProperty>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PhotoLoraInpaintInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// URL of image to use for inpainting. or img2img/// URL of image to use for inpainting. or img2img/// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo.png"
    pub image_url: String,
    /// The LoRAs to use for the image generation. You can use any number of LoRAs
    /// and they will be merged together to generate the final image./// The LoRAs to use for the image generation. You can use any number of LoRAs
    /// and they will be merged together to generate the final image.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The mask to area to Inpaint in./// The mask to area to Inpaint in./// "https://raw.githubusercontent.com/CompVis/latent-diffusion/main/data/inpainting_examples/overture-creations-5sI6fQgYIuo_mask.png"
    pub mask_url: String,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// LoRA Scale of the photo lora model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_lora_scale: Option<f64>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "A photo of a lion sitting on a stone bench"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of the model
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The strength to use for inpainting/image-to-image. Only used if the image_url is provided. 1.0 is completely remakes the image while 0.0 preserves the original.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Ben2OutputVideo {
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
    /// The generated video file./// The generated video file./// {"content_type":"video/mp4","url":"https://storage.googleapis.com/falserverless/gallery/Ben2/foreground.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RemeshingInput {
    /// Number of faces for remesh
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faces: Option<i64>,
    /// Merge duplicate vertices before exporting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge: Option<bool>,
    /// Path for the object file to be remeshed./// Path for the object file to be remeshed./// "https://huggingface.co/fal-ai/resources/resolve/main/inputs/example_obj.glb"
    pub object_url: String,
    /// Output format for the 3D model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// Preserve UVs during remeshing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_uvs: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NafnetInput {
    /// URL of image to be used for relighting/// URL of image to be used for relighting/// "https://storage.googleapis.com/falserverless/nafnet/blurry.png"
    pub image_url: String,
    /// seed to be used for generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PixArtSigmaOutput {
    /// Whether the generated images contain NSFW concepts.
    pub has_nsfw_concepts: Vec<bool>,
    /// The generated image files info.
    pub images: Vec<Image>,
    /// The prompt used for generating the image.
    pub prompt: String,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
    /// The timings of the different steps of the generation process.
    pub timings: Timings,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextToMusicRequest {
    /// Lyrics with optional formatting. You can use a newline to separate each line of lyrics. You can use two newlines to add a pause between lines. You can use double hash marks (##) at the beginning and end of the lyrics to add accompaniment. Maximum 600 characters./// Lyrics with optional formatting. You can use a newline to separate each line of lyrics. You can use two newlines to add a pause between lines. You can use double hash marks (##) at the beginning and end of the lyrics to add accompaniment. Maximum 600 characters./// "## Fast and Limitless   \n In the heart of the code, where dreams collide,   \n\nFAL's the name, taking tech for a ride.    \nGenerative media, blazing the trail,   \n\nFast inference power, we'll never fail.\n##"
    pub prompt: String,
    /// Reference song, should contain music and vocals. Must be a .wav or .mp3 file longer than 15 seconds./// Reference song, should contain music and vocals. Must be a .wav or .mp3 file longer than 15 seconds./// "https://fal.media/files/lion/OOTBTSlxKMH_E8H6hoSlb.mpga"
    pub reference_audio_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RealisticVisionTextToImageInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The list of LoRA weights to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<Option<LoraWeight>>>,
    /// The Realistic Vision model to use./// The Realistic Vision model to use./// "SG161222/Realistic_Vision_V6.0_B1_noVAE"
    /// "SG161222/RealVisXL_V4.0"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The negative prompt to use. Use it to address details that you don't want in the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "A hyperdetailed photograph of a Cat dressed as a mafia boss holding a fish walking down a Japanese fish market with an angry face, 8k resolution, best quality, beautiful photograph, dynamic lighting,"
    pub prompt: String,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
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
pub struct IllusionDiffusionOutput {
    /// The generated image file info.
    pub image: Image,
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IpAdapterFaceIdInput {
    /// The URL to the base 1.5 model. Default is SG161222/Realistic_Vision_V4.0_noVAE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_1_5_model_repo: Option<String>,
    /// The URL to the base SDXL model. Default is SG161222/RealVisXL_V3.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_sdxl_model_repo: Option<String>,
    /// The size of the face detection model. The higher the number the more accurate
    /// the detection will be but it will also take longer to run. The higher the number the more
    /// likely it will fail to find a face as well. Lower it if you are having trouble
    /// finding a face in the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_id_det_size: Option<i64>,
    /// An image of a face to match. If an image with a size of 640x640 is not provided, it will be scaled and cropped to that size./// An image of a face to match. If an image with a size of 640x640 is not provided, it will be scaled and cropped to that size./// "https://storage.googleapis.com/falserverless/model_tests/upscale/image%20(8).png"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_image_url: Option<String>,
    /// URL to zip archive with images of faces. The images embedding will be averaged to
    /// create a more accurate face id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_images_data_url: Option<String>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The height of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// The model type to use. 1_5 is the default and is recommended for most use cases./// The model type to use. 1_5 is the default and is recommended for most use cases./// "1_5-v1"
    /// "1_5-v1-plus"
    /// "1_5-v2-plus"
    /// "SDXL-v1"
    /// "SDXL-v2-plus"
    /// "1_5-auraface-v1"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "blurry, low resolution, bad, ugly, low quality, pixelated, interpolated, compression artifacts, noisey, grainy"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of inference steps to use for generating the image. The more steps
    /// the better the image will be but it will also take longer to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The number of samples for face id. The more samples the better the image will
    /// be but it will also take longer to generate. Default is 4.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_samples: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "Man cyberpunk, synthwave night city, futuristic, high quality, highly detailed, high resolution, sharp, hyper realistic, extremely detailed"
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time./// 42

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The width of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FastSVDOutput {
    /// Seed of the generated Image. It will be the same value of the one passed in the
    /// input or the randomly generated that was used in case none was passed.
    pub seed: i64,
    /// The generated video file.
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FluxProDepthControlInput {
    /// The control image URL to generate the depth map from./// The control image URL to generate the depth map from./// "https://fal.media/files/penguin/vt-SeIOweN7_oYBsvGO6t.png"
    pub control_image_url: String,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "A blackhole in space."
    pub prompt: String,
    /// The safety tolerance level for the generated image. 1 being the most strict and 5 being the most permissive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_tolerance: Option<String>,
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
pub struct MiniMaxTextToImageRequest {
    /// Aspect ratio of the generated image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// Number of images to generate (1-9)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// Text prompt for image generation (max 1500 characters)/// Text prompt for image generation (max 1500 characters)/// "men Dressing in white t shirt, full-body stand front view image, outdoor, Venice beach sign, full-body image, Los Angeles, Fashion photography of 90s, documentary, Film grain, photorealistic"
    pub prompt: String,
    /// Whether to enable automatic prompt optimization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_optimizer: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BGReplaceOutput {
    /// The generated images/// The generated images/// [{"content_type":"image/png","url":"https://storage.googleapis.com/falserverless/bria/bria_bg_replace_res.jpg"}]
    pub images: Vec<Image>,
    /// Seed value used for generation.
    pub seed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SAM2RLEOutput {
    /// Run Length Encoding of the mask.
    pub rle: RleProperty,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct KolorsInput {
    /// Enable safety checker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show
    /// you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small
    /// details (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small
    /// details (e.g. moustache, blurry, low resolution)./// "ugly, deformed, blurry"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to use for generating the image. Be as descriptive as possible
    /// for best results./// The prompt to use for generating the image. Be as descriptive as possible
    /// for best results./// "A young Chinese couple with fair skin, dressed in stylish sportswear, with the modern Beijing city skyline in the background. Facial details, clear pores, captured using the latest camera model, close-up shot, ultra-high quality, 8K, visual feast."
    /// "The image features four mythical beasts: Vermilion Bird, Black Tortoise, Azure Dragon, and White Tiger. The Vermilion Bird is at the top of the image, with feathers as red as fire and a tail as magnificent as a phoenix, its wings spreading like burning flames. The Black Tortoise is at the bottom, depicted as a giant turtle intertwined with a snake. Ancient runes adorn the turtle's shell, and the snake's eyes are cold and sharp. The Azure Dragon is on the right, its long body coiling in the sky, with jade-green scales, flowing whiskers, deer-like horns, and exhaling clouds and mist. The White Tiger is on the left, with a majestic posture, white fur with black stripes, piercing eyes, sharp teeth and claws, surrounded by vast mountains and grasslands."
    pub prompt: String,
    /// The scheduler to use for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// Seed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// If set to true, the function will wait for the image to be generated and
    /// uploaded before returning the response. This will increase the latency of
    /// the function but it allows you to get the image directly in the response
    /// without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Trajectory {
    /// X coordinate of the motion trajectory/// X coordinate of the motion trajectory/// 279
    pub x: i64,
    /// Y coordinate of the motion trajectory/// Y coordinate of the motion trajectory/// 219
    pub y: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageToImageTurboInput {
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://fal-cdn.batuhan-941.workers.dev/files/koala/-CQBCeIxrvPqrvt4FDY5n.jpeg"
    pub image_url: String,
    /// The name of the model to use./// The name of the model to use./// "stabilityai/sdxl-turbo"
    /// "stabilityai/sd-turbo"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "A cinematic shot of a baby cat wearing an intricate italian priest robe."
    pub prompt: String,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Ray2I2VOutput {
    /// URL of the generated video/// URL of the generated video/// {"url":"https://v3.fal.media/files/zebra/9aDde3Te2kuJYHdR0Kz8R_output.mp4"}
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageToImageFooocusInput {
    /// The list of embeddings to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeddings: Option<Vec<Option<Embedding>>>,
    /// If set to true, a smaller model will try to refine the output after it was processed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_refiner: Option<bool>,
    /// If set to true, the safety checker will be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// If set to true, the prompt will be expanded with additional prompts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_prompt: Option<bool>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The rescale factor for the CFG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_rescale: Option<f64>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image. Leave it none to automatically infer from the prompt image./// The size of the generated image. Leave it none to automatically infer from the prompt image./// null

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// The URL of the image to use as a starting point for the generation./// The URL of the image to use as a starting point for the generation./// "https://fal-cdn.batuhan-941.workers.dev/files/tiger/IExuP-WICqaIesLZAZPur.jpeg"
    pub image_url: String,
    /// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// The negative prompt to use.Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small details
    /// (e.g. moustache, blurry, low resolution)./// "cartoon, illustration, animation. face. male, female"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The prompt to use for generating the image. Be as descriptive as possible for best results./// The prompt to use for generating the image. Be as descriptive as possible for best results./// "an island near sea, with seagulls, moon shining over the sea, light house, boats int he background, fish flying over the sea"
    pub prompt: String,
    /// The version of the safety checker to use. v1 is the default CompVis safety checker. v2 uses a custom ViT model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_checker_version: Option<String>,
    /// The same seed and the same prompt given to the same version of Stable Diffusion
    /// will output the same image every time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// determines how much the generated image resembles the initial image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CATVTONOutput {
    /// The output image.
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SAM2AutomaticSegmentationInput {
    /// URL of the image to be automatically segmented/// URL of the image to be automatically segmented/// "https://raw.githubusercontent.com/facebookresearch/segment-anything-2/main/notebooks/images/truck.jpg"
    pub image_url: String,
    /// Minimum area of a mask region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_mask_region_area: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// Number of points to sample along each side of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points_per_side: Option<i64>,
    /// Threshold for predicted IOU score.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pred_iou_thresh: Option<f64>,
    /// Threshold for stability score.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability_score_thresh: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and uploaded
    /// before returning the response. This will increase the latency of the function but
    /// it allows you to get the image directly in the response without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TeedOutput {
    /// The edge map./// The edge map./// {"content_type":"image/png","height":2048,"url":"https://storage.googleapis.com/falserverless/model_tests/workflow_utils/teed_output.png","width":1246}
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VideoFormat {
    /// Video bitrate in bits per second
    pub bitrate: i64,
    /// Container format of the video
    pub container: String,
    /// Codec level (e.g., 4.1)
    pub level: f64,
    /// Pixel format used (e.g., 'yuv420p')
    pub pixel_format: String,
    /// Codec profile (e.g., 'main', 'high')
    pub profile: String,
    /// Video codec used (e.g., 'h264')
    pub video_codec: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WhisperChunk {
    /// Transcription of the chunk
    pub text: String,
    /// Start and end timestamp of the chunk
    pub timestamp: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DepthAnythingV2Input {
    /// URL of the image to process/// URL of the image to process/// "https://storage.googleapis.com/falserverless/model_tests/image_preprocessors/cat.png"
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct KolorsImg2ImgInput {
    /// Enable safety checker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_safety_checker: Option<bool>,
    /// The CFG (Classifier Free Guidance) scale is a measure of how close you want
    /// the model to stick to your prompt when looking for a related image to show
    /// you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance_scale: Option<f64>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<ImageSizeProperty>,
    /// URL of image to use for image to image/// URL of image to use for image to image/// "https://storage.googleapis.com/falserverless/model_tests/image_models/bunny_source.png"
    pub image_url: String,
    /// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small
    /// details (e.g. moustache, blurry, low resolution)./// The negative prompt to use. Use it to address details that you don't want
    /// in the image. This could be colors, objects, scenery and even the small
    /// details (e.g. moustache, blurry, low resolution)./// "ugly, deformed, blurry"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// The number of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_images: Option<i64>,
    /// The number of inference steps to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_inference_steps: Option<i64>,
    /// The format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// The prompt to generate an image from./// The prompt to generate an image from./// "high quality image of a capybara wearing sunglasses. In the background of the image there are trees, poles, grass and other objects. At the bottom of the object there is the road., 8k, highly detailed."
    pub prompt: String,
    /// The scheduler to use for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
    /// Seed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The strength to use for image-to-image. 1.0 is completely remakes the image while 0.0 preserves the original.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f64>,
    /// If set to true, the function will wait for the image to be generated and
    /// uploaded before returning the response. This will increase the latency of
    /// the function but it allows you to get the image directly in the response
    /// without going through the CDN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MiDaSInput {
    /// A parameter for the MiDaS detector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<f64>,
    /// Background threshold for the MiDaS detector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_threshold: Option<f64>,
    /// URL of the image to process/// URL of the image to process/// "https://storage.googleapis.com/falserverless/model_tests/image_preprocessors/cat.png"
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DepthMapOutput {
    /// The depth map.
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PiDiInput {
    /// Whether to apply the filter to the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_filter: Option<bool>,
    /// URL of the image to process/// URL of the image to process/// "https://storage.googleapis.com/falserverless/model_tests/image_preprocessors/cat.png"
    pub image_url: String,
    /// Whether to use the safe version of the Pidi detector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe: Option<bool>,
    /// Whether to use the scribble version of the Pidi detector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scribble: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OutputModel {
    /// Generated video prompt/// Generated video prompt/// "A futuristic city glows softly at dusk, captured with smooth gimbal movements and a slow burn pacing, enhanced by subtle holographic overlays."
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MiniCPMV26Output {
    /// Response from the model
    pub output: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CodeformerInput {
    /// Should faces etc should be aligned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aligned: Option<bool>,
    /// Should faces be upscaled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_upscale: Option<bool>,
    /// Weight of the fidelity factor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fidelity: Option<f64>,
    /// URL of image to be used for relighting/// URL of image to be used for relighting/// "https://storage.googleapis.com/falserverless/model_tests/codeformer/codeformer_poor_1.jpeg"
    pub image_url: String,
    /// Should only center face be restored
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_center_face: Option<bool>,
    /// Random seed for reproducible generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Upscaling factor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upscaling: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MuseTalkInput {
    /// URL of the audio/// URL of the audio/// "https://raw.githubusercontent.com/TMElyralab/MuseTalk/main/data/audio/sun.wav"
    pub audio_url: String,
    /// URL of the source video/// URL of the source video/// "https://raw.githubusercontent.com/TMElyralab/MuseTalk/main/data/video/sun.mp4"
    pub source_video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CompositeImageInput {
    /// Input image url./// Input image url./// "https://storage.googleapis.com/falserverless/model_tests/workflow_utils/mask_input.png"
    pub background_image_url: String,
    /// Optional mask image url./// Optional mask image url./// "https://storage.googleapis.com/falserverless/model_tests/workflow_utils/mask_input.png"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_image_url: Option<String>,
    /// Overlay image url./// Overlay image url./// "https://storage.googleapis.com/falserverless/model_tests/workflow_utils/mask_input.png"
    pub overlay_image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FrenchRequest {
    pub prompt: String,
    /// Voice ID for the desired voice./// Voice ID for the desired voice./// "ff_siwis"
    pub voice: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputV2 {
    /// URL of the image to remove background from/// URL of the image to remove background from/// "https://fal.media/files/panda/K5Rndvzmn1j-OI1VZXDVd.jpeg"
    pub image_url: String,
    /// Model to use for background removal.
    /// The 'General Use (Light)' model is the original model used in the BiRefNet repository.
    /// The 'General Use (Light)' model is the original model used in the BiRefNet repository but trained with 2K images.
    /// The 'General Use (Heavy)' model is a slower but more accurate model.
    /// The 'Matting' model is a model trained specifically for matting images.
    /// The 'Portrait' model is a model trained specifically for portrait images.
    /// The 'General Use (Light)' model is recommended for most use cases.
    ///
    /// The corresponding models are as follows:
    /// - 'General Use (Light)': BiRefNet-DIS_ep580.pth
    /// - 'General Use (Heavy)': BiRefNet-massive-epoch_240.pth
    /// - 'Portrait': BiRefNet-portrait-TR_P3M_10k-epoch_120.pth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The resolution to operate on. The higher the resolution, the more accurate the output will be for high res input images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_resolution: Option<String>,
    /// The format of the output image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// Whether to output the mask used to remove the background
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_mask: Option<bool>,
    /// Whether to refine the foreground using the estimated mask
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refine_foreground: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ImageWithTextInput {
    /// The URL of the image to be processed./// The URL of the image to be processed./// "https://huggingface.co/datasets/huggingface/documentation-images/resolve/main/transformers/tasks/car.jpg"
    /// "http://ecx.images-amazon.com/images/I/51UUzBDAMsL.jpg"
    pub image_url: String,
    /// Text input for the task
    pub text_input: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CannyOutput {
    /// Image with edges detected using the Canny algorithm
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EyeCorrectInput {
    /// The URL of the video to correct
    pub video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LLavaOutput {
    /// Generated output/// Generated output/// "Leonardo da Vinci"
    pub output: String,
    /// Whether the output is partial
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PointPrompt {
    /// The frame index to interact with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_index: Option<i64>,
    /// Label of the prompt. 1 for foreground, 0 for background
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<i64>,
    /// X Coordinate of the prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    /// Y Coordinate of the prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AnimatediffLCMOutput {
    /// The seed used to generate the video.
    pub seed: i64,
    /// Generated video file.
    pub video: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AudioOutput {
    /// The generated audio./// The generated audio./// {"content_type":"application/octet-stream","file_name":"mmaudio_input.flac","file_size":1001342,"url":"https://storage.googleapis.com/falserverless/model_tests/video_models/mmaudio_output.flac"}
    pub audio: File,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SwinSrOutput {
    /// The generated image file info.
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum RleProperty {
    #[default]
    String(String),
    Array(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum FillImageUrlProperty {
    #[default]
    Array(Vec<String>),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum SeedProperty {
    #[default]
    Integer(i64),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum MediaProperty {
    #[default]
    Video(Video),
    Audio(Audio),
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Timings {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub ty: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum ErrorProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum BboxConditionProperty {
    #[default]
    Array(Vec<i64>),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum IpAdapterImageUrlProperty {
    #[default]
    String(String),
    Array(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum SystemPromptProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum VideoSizeProperty {
    #[default]
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

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum TopAlignProperty {
    #[default]
    String(String),
    Number(f64),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum AspectRatioProperty {
    #[default]
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

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum SpeakerIdProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum LanguageCodeProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum DurationSecondsProperty {
    #[default]
    Number(f64),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum LoopModeProperty {
    #[default]
    #[serde(rename = "pingpong")]
    Pingpong,
    #[serde(rename = "loop")]
    Loop,
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum LeftAlignProperty {
    #[default]
    String(String),
    Number(f64),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum ImageSizeProperty {
    #[default]
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

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum AddonsProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HasNsfwConcepts {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub ty: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum ScaleProperty {
    #[default]
    Object(HashMap<String, serde_json::Value>),
    Number(f64),
}
