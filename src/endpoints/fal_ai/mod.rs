#[cfg(any(
    feature = "endpoints_fal-ai_amt-interpolation",
    feature = "endpoints_fal-ai_amt-interpolation_frame-interpolation"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_amt-interpolation",
        feature = "endpoints_fal-ai_amt-interpolation_frame-interpolation"
    )))
)]
pub mod amt_interpolation;
#[cfg(any(feature = "endpoints_fal-ai_animatediff-sparsectrl-lcm"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_animatediff-sparsectrl-lcm")))
)]
pub mod animatediff_sparsectrl_lcm;
#[cfg(any(
    feature = "endpoints_fal-ai_animatediff-v2v",
    feature = "endpoints_fal-ai_animatediff-v2v_turbo"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_animatediff-v2v",
        feature = "endpoints_fal-ai_animatediff-v2v_turbo"
    )))
)]
pub mod animatediff_v2v;
#[cfg(any(
    feature = "endpoints_fal-ai_any-llm",
    feature = "endpoints_fal-ai_any-llm_vision"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_any-llm",
        feature = "endpoints_fal-ai_any-llm_vision"
    )))
)]
pub mod any_llm;
#[cfg(any(feature = "endpoints_fal-ai_aura-flow"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_aura-flow"))))]
pub mod aura_flow;
#[cfg(any(feature = "endpoints_fal-ai_aura-sr"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_aura-sr"))))]
pub mod aura_sr;
#[cfg(any(feature = "endpoints_fal-ai_auto-caption"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_auto-caption"))))]
pub mod auto_caption;
#[cfg(any(
    feature = "endpoints_fal-ai_ben",
    feature = "endpoints_fal-ai_ben_v2",
    feature = "endpoints_fal-ai_ben_v2_image",
    feature = "endpoints_fal-ai_ben_v2_video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_ben",
        feature = "endpoints_fal-ai_ben_v2",
        feature = "endpoints_fal-ai_ben_v2_image",
        feature = "endpoints_fal-ai_ben_v2_video"
    )))
)]
pub mod ben;
#[cfg(any(
    feature = "endpoints_fal-ai_birefnet",
    feature = "endpoints_fal-ai_birefnet_v2"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_birefnet",
        feature = "endpoints_fal-ai_birefnet_v2"
    )))
)]
pub mod birefnet;
#[cfg(any(
    feature = "endpoints_fal-ai_bria",
    feature = "endpoints_fal-ai_bria_background",
    feature = "endpoints_fal-ai_bria_background_remove",
    feature = "endpoints_fal-ai_bria_background_replace",
    feature = "endpoints_fal-ai_bria_eraser",
    feature = "endpoints_fal-ai_bria_expand",
    feature = "endpoints_fal-ai_bria_genfill",
    feature = "endpoints_fal-ai_bria_product-shot",
    feature = "endpoints_fal-ai_bria_reimagine",
    feature = "endpoints_fal-ai_bria_text-to-image",
    feature = "endpoints_fal-ai_bria_text-to-image_base",
    feature = "endpoints_fal-ai_bria_text-to-image_fast",
    feature = "endpoints_fal-ai_bria_text-to-image_hd",
    feature = "endpoints_fal-ai_bria_upscale"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_bria",
        feature = "endpoints_fal-ai_bria_background",
        feature = "endpoints_fal-ai_bria_background_remove",
        feature = "endpoints_fal-ai_bria_background_replace",
        feature = "endpoints_fal-ai_bria_eraser",
        feature = "endpoints_fal-ai_bria_expand",
        feature = "endpoints_fal-ai_bria_genfill",
        feature = "endpoints_fal-ai_bria_product-shot",
        feature = "endpoints_fal-ai_bria_reimagine",
        feature = "endpoints_fal-ai_bria_text-to-image",
        feature = "endpoints_fal-ai_bria_text-to-image_base",
        feature = "endpoints_fal-ai_bria_text-to-image_fast",
        feature = "endpoints_fal-ai_bria_text-to-image_hd",
        feature = "endpoints_fal-ai_bria_upscale"
    )))
)]
pub mod bria;
#[cfg(any(feature = "endpoints_fal-ai_cat-vton"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_cat-vton"))))]
pub mod cat_vton;
#[cfg(any(feature = "endpoints_fal-ai_ccsr"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_ccsr"))))]
pub mod ccsr;
#[cfg(any(feature = "endpoints_fal-ai_clarity-upscaler"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_clarity-upscaler"))))]
pub mod clarity_upscaler;
#[cfg(any(feature = "endpoints_fal-ai_codeformer"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_codeformer"))))]
pub mod codeformer;
#[cfg(any(
    feature = "endpoints_fal-ai_cogvideox-5b",
    feature = "endpoints_fal-ai_cogvideox-5b_image-to-video",
    feature = "endpoints_fal-ai_cogvideox-5b_video-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_cogvideox-5b",
        feature = "endpoints_fal-ai_cogvideox-5b_image-to-video",
        feature = "endpoints_fal-ai_cogvideox-5b_video-to-video"
    )))
)]
pub mod cogvideox_5b;
#[cfg(any(feature = "endpoints_fal-ai_cogview4"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_cogview4"))))]
pub mod cogview4;
#[cfg(any(feature = "endpoints_fal-ai_controlnext"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_controlnext"))))]
pub mod controlnext;
#[cfg(any(feature = "endpoints_fal-ai_creative-upscaler"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_creative-upscaler"))))]
pub mod creative_upscaler;
#[cfg(any(feature = "endpoints_fal-ai_ddcolor"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_ddcolor"))))]
pub mod ddcolor;
#[cfg(any(feature = "endpoints_fal-ai_diffrhythm"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_diffrhythm"))))]
pub mod diffrhythm;
#[cfg(any(feature = "endpoints_fal-ai_diffusion-edge"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_diffusion-edge"))))]
pub mod diffusion_edge;
#[cfg(any(
    feature = "endpoints_fal-ai_docres",
    feature = "endpoints_fal-ai_docres_dewarp"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_docres",
        feature = "endpoints_fal-ai_docres_dewarp"
    )))
)]
pub mod docres;
#[cfg(any(feature = "endpoints_fal-ai_drct-super-resolution"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_drct-super-resolution")))
)]
pub mod drct_super_resolution;
#[cfg(any(
    feature = "endpoints_fal-ai_dreamshaper",
    feature = "endpoints_fal-ai_dreamshaper_image-to-image",
    feature = "endpoints_fal-ai_dreamshaper_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_dreamshaper",
        feature = "endpoints_fal-ai_dreamshaper_image-to-image",
        feature = "endpoints_fal-ai_dreamshaper_inpainting"
    )))
)]
pub mod dreamshaper;
#[cfg(any(feature = "endpoints_fal-ai_dubbing"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_dubbing"))))]
pub mod dubbing;
#[cfg(any(feature = "endpoints_fal-ai_dwpose"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_dwpose"))))]
pub mod dwpose;
#[cfg(any(
    feature = "endpoints_fal-ai_elevenlabs",
    feature = "endpoints_fal-ai_elevenlabs_audio-isolation",
    feature = "endpoints_fal-ai_elevenlabs_sound-effects",
    feature = "endpoints_fal-ai_elevenlabs_speech-to-text",
    feature = "endpoints_fal-ai_elevenlabs_tts",
    feature = "endpoints_fal-ai_elevenlabs_tts_multilingual-v2",
    feature = "endpoints_fal-ai_elevenlabs_tts_turbo-v2-5"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_elevenlabs",
        feature = "endpoints_fal-ai_elevenlabs_audio-isolation",
        feature = "endpoints_fal-ai_elevenlabs_sound-effects",
        feature = "endpoints_fal-ai_elevenlabs_speech-to-text",
        feature = "endpoints_fal-ai_elevenlabs_tts",
        feature = "endpoints_fal-ai_elevenlabs_tts_multilingual-v2",
        feature = "endpoints_fal-ai_elevenlabs_tts_turbo-v2-5"
    )))
)]
pub mod elevenlabs;
#[cfg(any(feature = "endpoints_fal-ai_era-3d"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_era-3d"))))]
pub mod era_3d;
#[cfg(any(feature = "endpoints_fal-ai_esrgan"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_esrgan"))))]
pub mod esrgan;
#[cfg(any(feature = "endpoints_fal-ai_evf-sam"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_evf-sam"))))]
pub mod evf_sam;
#[cfg(any(feature = "endpoints_fal-ai_eye-correct"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_eye-correct"))))]
pub mod eye_correct;
#[cfg(any(feature = "endpoints_fal-ai_f5-tts"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_f5-tts"))))]
pub mod f5_tts;
#[cfg(any(feature = "endpoints_fal-ai_face-to-sticker"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_face-to-sticker"))))]
pub mod face_to_sticker;
#[cfg(any(
    feature = "endpoints_fal-ai_fast-animatediff",
    feature = "endpoints_fal-ai_fast-animatediff_text-to-video",
    feature = "endpoints_fal-ai_fast-animatediff_turbo",
    feature = "endpoints_fal-ai_fast-animatediff_turbo_text-to-video",
    feature = "endpoints_fal-ai_fast-animatediff_turbo_video-to-video",
    feature = "endpoints_fal-ai_fast-animatediff_video-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_fast-animatediff",
        feature = "endpoints_fal-ai_fast-animatediff_text-to-video",
        feature = "endpoints_fal-ai_fast-animatediff_turbo",
        feature = "endpoints_fal-ai_fast-animatediff_turbo_text-to-video",
        feature = "endpoints_fal-ai_fast-animatediff_turbo_video-to-video",
        feature = "endpoints_fal-ai_fast-animatediff_video-to-video"
    )))
)]
pub mod fast_animatediff;
#[cfg(any(
    feature = "endpoints_fal-ai_fast-fooocus-sdxl",
    feature = "endpoints_fal-ai_fast-fooocus-sdxl_image-to-image",
    feature = "endpoints_fal-ai_fast-fooocus-sdxl_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_fast-fooocus-sdxl",
        feature = "endpoints_fal-ai_fast-fooocus-sdxl_image-to-image",
        feature = "endpoints_fal-ai_fast-fooocus-sdxl_inpainting"
    )))
)]
pub mod fast_fooocus_sdxl;
#[cfg(any(
    feature = "endpoints_fal-ai_fast-lcm-diffusion",
    feature = "endpoints_fal-ai_fast-lcm-diffusion_image-to-image",
    feature = "endpoints_fal-ai_fast-lcm-diffusion_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_fast-lcm-diffusion",
        feature = "endpoints_fal-ai_fast-lcm-diffusion_image-to-image",
        feature = "endpoints_fal-ai_fast-lcm-diffusion_inpainting"
    )))
)]
pub mod fast_lcm_diffusion;
#[cfg(any(
    feature = "endpoints_fal-ai_fast-lightning-sdxl",
    feature = "endpoints_fal-ai_fast-lightning-sdxl_image-to-image",
    feature = "endpoints_fal-ai_fast-lightning-sdxl_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_fast-lightning-sdxl",
        feature = "endpoints_fal-ai_fast-lightning-sdxl_image-to-image",
        feature = "endpoints_fal-ai_fast-lightning-sdxl_inpainting"
    )))
)]
pub mod fast_lightning_sdxl;
#[cfg(any(
    feature = "endpoints_fal-ai_fast-sdxl",
    feature = "endpoints_fal-ai_fast-sdxl_image-to-image",
    feature = "endpoints_fal-ai_fast-sdxl_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_fast-sdxl",
        feature = "endpoints_fal-ai_fast-sdxl_image-to-image",
        feature = "endpoints_fal-ai_fast-sdxl_inpainting"
    )))
)]
pub mod fast_sdxl;
#[cfg(any(
    feature = "endpoints_fal-ai_fast-sdxl-controlnet-canny",
    feature = "endpoints_fal-ai_fast-sdxl-controlnet-canny_image-to-image",
    feature = "endpoints_fal-ai_fast-sdxl-controlnet-canny_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_fast-sdxl-controlnet-canny",
        feature = "endpoints_fal-ai_fast-sdxl-controlnet-canny_image-to-image",
        feature = "endpoints_fal-ai_fast-sdxl-controlnet-canny_inpainting"
    )))
)]
pub mod fast_sdxl_controlnet_canny;
#[cfg(any(
    feature = "endpoints_fal-ai_fast-svd",
    feature = "endpoints_fal-ai_fast-svd_text-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_fast-svd",
        feature = "endpoints_fal-ai_fast-svd_text-to-video"
    )))
)]
pub mod fast_svd;
#[cfg(any(
    feature = "endpoints_fal-ai_fast-svd-lcm",
    feature = "endpoints_fal-ai_fast-svd-lcm_text-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_fast-svd-lcm",
        feature = "endpoints_fal-ai_fast-svd-lcm_text-to-video"
    )))
)]
pub mod fast_svd_lcm;
#[cfg(any(
    feature = "endpoints_fal-ai_fast-turbo-diffusion",
    feature = "endpoints_fal-ai_fast-turbo-diffusion_image-to-image",
    feature = "endpoints_fal-ai_fast-turbo-diffusion_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_fast-turbo-diffusion",
        feature = "endpoints_fal-ai_fast-turbo-diffusion_image-to-image",
        feature = "endpoints_fal-ai_fast-turbo-diffusion_inpainting"
    )))
)]
pub mod fast_turbo_diffusion;
#[cfg(any(
    feature = "endpoints_fal-ai_ffmpeg-api",
    feature = "endpoints_fal-ai_ffmpeg-api_compose",
    feature = "endpoints_fal-ai_ffmpeg-api_metadata",
    feature = "endpoints_fal-ai_ffmpeg-api_waveform"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_ffmpeg-api",
        feature = "endpoints_fal-ai_ffmpeg-api_compose",
        feature = "endpoints_fal-ai_ffmpeg-api_metadata",
        feature = "endpoints_fal-ai_ffmpeg-api_waveform"
    )))
)]
pub mod ffmpeg_api;
#[cfg(any(
    feature = "endpoints_fal-ai_florence-2-large",
    feature = "endpoints_fal-ai_florence-2-large_caption",
    feature = "endpoints_fal-ai_florence-2-large_caption-to-phrase-grounding",
    feature = "endpoints_fal-ai_florence-2-large_dense-region-caption",
    feature = "endpoints_fal-ai_florence-2-large_detailed-caption",
    feature = "endpoints_fal-ai_florence-2-large_more-detailed-caption",
    feature = "endpoints_fal-ai_florence-2-large_object-detection",
    feature = "endpoints_fal-ai_florence-2-large_ocr",
    feature = "endpoints_fal-ai_florence-2-large_ocr-with-region",
    feature = "endpoints_fal-ai_florence-2-large_open-vocabulary-detection",
    feature = "endpoints_fal-ai_florence-2-large_referring-expression-segmentation",
    feature = "endpoints_fal-ai_florence-2-large_region-proposal",
    feature = "endpoints_fal-ai_florence-2-large_region-to-category",
    feature = "endpoints_fal-ai_florence-2-large_region-to-description",
    feature = "endpoints_fal-ai_florence-2-large_region-to-segmentation"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_florence-2-large",
        feature = "endpoints_fal-ai_florence-2-large_caption",
        feature = "endpoints_fal-ai_florence-2-large_caption-to-phrase-grounding",
        feature = "endpoints_fal-ai_florence-2-large_dense-region-caption",
        feature = "endpoints_fal-ai_florence-2-large_detailed-caption",
        feature = "endpoints_fal-ai_florence-2-large_more-detailed-caption",
        feature = "endpoints_fal-ai_florence-2-large_object-detection",
        feature = "endpoints_fal-ai_florence-2-large_ocr",
        feature = "endpoints_fal-ai_florence-2-large_ocr-with-region",
        feature = "endpoints_fal-ai_florence-2-large_open-vocabulary-detection",
        feature = "endpoints_fal-ai_florence-2-large_referring-expression-segmentation",
        feature = "endpoints_fal-ai_florence-2-large_region-proposal",
        feature = "endpoints_fal-ai_florence-2-large_region-to-category",
        feature = "endpoints_fal-ai_florence-2-large_region-to-description",
        feature = "endpoints_fal-ai_florence-2-large_region-to-segmentation"
    )))
)]
pub mod florence_2_large;
#[cfg(any(feature = "endpoints_fal-ai_flowedit"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_flowedit"))))]
pub mod flowedit;
#[cfg(any(
    feature = "endpoints_fal-ai_flux",
    feature = "endpoints_fal-ai_flux_dev",
    feature = "endpoints_fal-ai_flux_dev_image-to-image",
    feature = "endpoints_fal-ai_flux_dev_redux",
    feature = "endpoints_fal-ai_flux_schnell",
    feature = "endpoints_fal-ai_flux_schnell_redux"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_flux",
        feature = "endpoints_fal-ai_flux_dev",
        feature = "endpoints_fal-ai_flux_dev_image-to-image",
        feature = "endpoints_fal-ai_flux_dev_redux",
        feature = "endpoints_fal-ai_flux_schnell",
        feature = "endpoints_fal-ai_flux_schnell_redux"
    )))
)]
pub mod flux;
#[cfg(any(
    feature = "endpoints_fal-ai_flux-control-lora-canny",
    feature = "endpoints_fal-ai_flux-control-lora-canny_image-to-image"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_flux-control-lora-canny",
        feature = "endpoints_fal-ai_flux-control-lora-canny_image-to-image"
    )))
)]
pub mod flux_control_lora_canny;
#[cfg(any(
    feature = "endpoints_fal-ai_flux-control-lora-depth",
    feature = "endpoints_fal-ai_flux-control-lora-depth_image-to-image"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_flux-control-lora-depth",
        feature = "endpoints_fal-ai_flux-control-lora-depth_image-to-image"
    )))
)]
pub mod flux_control_lora_depth;
#[cfg(any(feature = "endpoints_fal-ai_flux-differential-diffusion"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_flux-differential-diffusion")))
)]
pub mod flux_differential_diffusion;
#[cfg(any(
    feature = "endpoints_fal-ai_flux-general",
    feature = "endpoints_fal-ai_flux-general_differential-diffusion",
    feature = "endpoints_fal-ai_flux-general_image-to-image",
    feature = "endpoints_fal-ai_flux-general_inpainting",
    feature = "endpoints_fal-ai_flux-general_rf-inversion"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_flux-general",
        feature = "endpoints_fal-ai_flux-general_differential-diffusion",
        feature = "endpoints_fal-ai_flux-general_image-to-image",
        feature = "endpoints_fal-ai_flux-general_inpainting",
        feature = "endpoints_fal-ai_flux-general_rf-inversion"
    )))
)]
pub mod flux_general;
#[cfg(any(
    feature = "endpoints_fal-ai_flux-lora",
    feature = "endpoints_fal-ai_flux-lora_image-to-image",
    feature = "endpoints_fal-ai_flux-lora_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_flux-lora",
        feature = "endpoints_fal-ai_flux-lora_image-to-image",
        feature = "endpoints_fal-ai_flux-lora_inpainting"
    )))
)]
pub mod flux_lora;
#[cfg(any(feature = "endpoints_fal-ai_flux-lora-canny"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_flux-lora-canny"))))]
pub mod flux_lora_canny;
#[cfg(any(feature = "endpoints_fal-ai_flux-lora-depth"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_flux-lora-depth"))))]
pub mod flux_lora_depth;
#[cfg(any(feature = "endpoints_fal-ai_flux-lora-fast-training"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_flux-lora-fast-training")))
)]
pub mod flux_lora_fast_training;
#[cfg(any(feature = "endpoints_fal-ai_flux-lora-fill"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_flux-lora-fill"))))]
pub mod flux_lora_fill;
#[cfg(any(feature = "endpoints_fal-ai_flux-lora-portrait-trainer"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_flux-lora-portrait-trainer")))
)]
pub mod flux_lora_portrait_trainer;
#[cfg(any(
    feature = "endpoints_fal-ai_flux-pro",
    feature = "endpoints_fal-ai_flux-pro_finetuned",
    feature = "endpoints_fal-ai_flux-pro_new",
    feature = "endpoints_fal-ai_flux-pro_v1",
    feature = "endpoints_fal-ai_flux-pro_v1-1",
    feature = "endpoints_fal-ai_flux-pro_v1-1-ultra",
    feature = "endpoints_fal-ai_flux-pro_v1-1-ultra-finetuned",
    feature = "endpoints_fal-ai_flux-pro_v1-1-ultra_redux",
    feature = "endpoints_fal-ai_flux-pro_v1-1_redux",
    feature = "endpoints_fal-ai_flux-pro_v1_canny",
    feature = "endpoints_fal-ai_flux-pro_v1_canny-finetuned",
    feature = "endpoints_fal-ai_flux-pro_v1_depth",
    feature = "endpoints_fal-ai_flux-pro_v1_depth-finetuned",
    feature = "endpoints_fal-ai_flux-pro_v1_fill",
    feature = "endpoints_fal-ai_flux-pro_v1_fill-finetuned",
    feature = "endpoints_fal-ai_flux-pro_v1_outpaint",
    feature = "endpoints_fal-ai_flux-pro_v1_redux"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_flux-pro",
        feature = "endpoints_fal-ai_flux-pro_finetuned",
        feature = "endpoints_fal-ai_flux-pro_new",
        feature = "endpoints_fal-ai_flux-pro_v1",
        feature = "endpoints_fal-ai_flux-pro_v1-1",
        feature = "endpoints_fal-ai_flux-pro_v1-1-ultra",
        feature = "endpoints_fal-ai_flux-pro_v1-1-ultra-finetuned",
        feature = "endpoints_fal-ai_flux-pro_v1-1-ultra_redux",
        feature = "endpoints_fal-ai_flux-pro_v1-1_redux",
        feature = "endpoints_fal-ai_flux-pro_v1_canny",
        feature = "endpoints_fal-ai_flux-pro_v1_canny-finetuned",
        feature = "endpoints_fal-ai_flux-pro_v1_depth",
        feature = "endpoints_fal-ai_flux-pro_v1_depth-finetuned",
        feature = "endpoints_fal-ai_flux-pro_v1_fill",
        feature = "endpoints_fal-ai_flux-pro_v1_fill-finetuned",
        feature = "endpoints_fal-ai_flux-pro_v1_outpaint",
        feature = "endpoints_fal-ai_flux-pro_v1_redux"
    )))
)]
pub mod flux_pro;
#[cfg(any(feature = "endpoints_fal-ai_flux-pro-trainer"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_flux-pro-trainer"))))]
pub mod flux_pro_trainer;
#[cfg(any(feature = "endpoints_fal-ai_flux-pulid"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_flux-pulid"))))]
pub mod flux_pulid;
#[cfg(any(feature = "endpoints_fal-ai_flux-subject"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_flux-subject"))))]
pub mod flux_subject;
#[cfg(any(
    feature = "endpoints_fal-ai_fooocus",
    feature = "endpoints_fal-ai_fooocus_image-prompt",
    feature = "endpoints_fal-ai_fooocus_inpaint",
    feature = "endpoints_fal-ai_fooocus_upscale-or-vary"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_fooocus",
        feature = "endpoints_fal-ai_fooocus_image-prompt",
        feature = "endpoints_fal-ai_fooocus_inpaint",
        feature = "endpoints_fal-ai_fooocus_upscale-or-vary"
    )))
)]
pub mod fooocus;
#[cfg(any(
    feature = "endpoints_fal-ai_got-ocr",
    feature = "endpoints_fal-ai_got-ocr_v2"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_got-ocr",
        feature = "endpoints_fal-ai_got-ocr_v2"
    )))
)]
pub mod got_ocr;
#[cfg(any(
    feature = "endpoints_fal-ai_hunyuan-video",
    feature = "endpoints_fal-ai_hunyuan-video_video-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_hunyuan-video",
        feature = "endpoints_fal-ai_hunyuan-video_video-to-video"
    )))
)]
pub mod hunyuan_video;
#[cfg(any(feature = "endpoints_fal-ai_hunyuan-video-image-to-video"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_hunyuan-video-image-to-video")))
)]
pub mod hunyuan_video_image_to_video;
#[cfg(any(feature = "endpoints_fal-ai_hunyuan-video-img2vid-lora"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_hunyuan-video-img2vid-lora")))
)]
pub mod hunyuan_video_img2vid_lora;
#[cfg(any(feature = "endpoints_fal-ai_hunyuan-video-lora"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_hunyuan-video-lora"))))]
pub mod hunyuan_video_lora;
#[cfg(any(feature = "endpoints_fal-ai_hunyuan-video-lora-training"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_hunyuan-video-lora-training")))
)]
pub mod hunyuan_video_lora_training;
#[cfg(any(
    feature = "endpoints_fal-ai_hyper3d",
    feature = "endpoints_fal-ai_hyper3d_rodin"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_hyper3d",
        feature = "endpoints_fal-ai_hyper3d_rodin"
    )))
)]
pub mod hyper3d;
#[cfg(any(
    feature = "endpoints_fal-ai_hyper-sdxl",
    feature = "endpoints_fal-ai_hyper-sdxl_image-to-image",
    feature = "endpoints_fal-ai_hyper-sdxl_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_hyper-sdxl",
        feature = "endpoints_fal-ai_hyper-sdxl_image-to-image",
        feature = "endpoints_fal-ai_hyper-sdxl_inpainting"
    )))
)]
pub mod hyper_sdxl;
#[cfg(any(feature = "endpoints_fal-ai_iclight-v2"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_iclight-v2"))))]
pub mod iclight_v2;
#[cfg(any(
    feature = "endpoints_fal-ai_ideogram",
    feature = "endpoints_fal-ai_ideogram_upscale",
    feature = "endpoints_fal-ai_ideogram_v2",
    feature = "endpoints_fal-ai_ideogram_v2_edit",
    feature = "endpoints_fal-ai_ideogram_v2_remix",
    feature = "endpoints_fal-ai_ideogram_v2_turbo",
    feature = "endpoints_fal-ai_ideogram_v2_turbo_edit",
    feature = "endpoints_fal-ai_ideogram_v2_turbo_remix",
    feature = "endpoints_fal-ai_ideogram_v2a",
    feature = "endpoints_fal-ai_ideogram_v2a_remix",
    feature = "endpoints_fal-ai_ideogram_v2a_turbo",
    feature = "endpoints_fal-ai_ideogram_v2a_turbo_remix"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_ideogram",
        feature = "endpoints_fal-ai_ideogram_upscale",
        feature = "endpoints_fal-ai_ideogram_v2",
        feature = "endpoints_fal-ai_ideogram_v2_edit",
        feature = "endpoints_fal-ai_ideogram_v2_remix",
        feature = "endpoints_fal-ai_ideogram_v2_turbo",
        feature = "endpoints_fal-ai_ideogram_v2_turbo_edit",
        feature = "endpoints_fal-ai_ideogram_v2_turbo_remix",
        feature = "endpoints_fal-ai_ideogram_v2a",
        feature = "endpoints_fal-ai_ideogram_v2a_remix",
        feature = "endpoints_fal-ai_ideogram_v2a_turbo",
        feature = "endpoints_fal-ai_ideogram_v2a_turbo_remix"
    )))
)]
pub mod ideogram;
#[cfg(any(feature = "endpoints_fal-ai_illusion-diffusion"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_illusion-diffusion"))))]
pub mod illusion_diffusion;
#[cfg(any(
    feature = "endpoints_fal-ai_image-preprocessors",
    feature = "endpoints_fal-ai_image-preprocessors_canny",
    feature = "endpoints_fal-ai_image-preprocessors_depth-anything",
    feature = "endpoints_fal-ai_image-preprocessors_depth-anything_v2",
    feature = "endpoints_fal-ai_image-preprocessors_hed",
    feature = "endpoints_fal-ai_image-preprocessors_lineart",
    feature = "endpoints_fal-ai_image-preprocessors_midas",
    feature = "endpoints_fal-ai_image-preprocessors_mlsd",
    feature = "endpoints_fal-ai_image-preprocessors_pidi",
    feature = "endpoints_fal-ai_image-preprocessors_sam",
    feature = "endpoints_fal-ai_image-preprocessors_scribble",
    feature = "endpoints_fal-ai_image-preprocessors_teed",
    feature = "endpoints_fal-ai_image-preprocessors_zoe"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_image-preprocessors",
        feature = "endpoints_fal-ai_image-preprocessors_canny",
        feature = "endpoints_fal-ai_image-preprocessors_depth-anything",
        feature = "endpoints_fal-ai_image-preprocessors_depth-anything_v2",
        feature = "endpoints_fal-ai_image-preprocessors_hed",
        feature = "endpoints_fal-ai_image-preprocessors_lineart",
        feature = "endpoints_fal-ai_image-preprocessors_midas",
        feature = "endpoints_fal-ai_image-preprocessors_mlsd",
        feature = "endpoints_fal-ai_image-preprocessors_pidi",
        feature = "endpoints_fal-ai_image-preprocessors_sam",
        feature = "endpoints_fal-ai_image-preprocessors_scribble",
        feature = "endpoints_fal-ai_image-preprocessors_teed",
        feature = "endpoints_fal-ai_image-preprocessors_zoe"
    )))
)]
pub mod image_preprocessors;
#[cfg(any(
    feature = "endpoints_fal-ai_imagen3",
    feature = "endpoints_fal-ai_imagen3_fast",
    feature = "endpoints_fal-ai_imagen3_subject-reference"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_imagen3",
        feature = "endpoints_fal-ai_imagen3_fast",
        feature = "endpoints_fal-ai_imagen3_subject-reference"
    )))
)]
pub mod imagen3;
#[cfg(any(
    feature = "endpoints_fal-ai_imageutils",
    feature = "endpoints_fal-ai_imageutils_depth",
    feature = "endpoints_fal-ai_imageutils_esrgan",
    feature = "endpoints_fal-ai_imageutils_marigold-depth",
    feature = "endpoints_fal-ai_imageutils_nsfw",
    feature = "endpoints_fal-ai_imageutils_rembg",
    feature = "endpoints_fal-ai_imageutils_sam"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_imageutils",
        feature = "endpoints_fal-ai_imageutils_depth",
        feature = "endpoints_fal-ai_imageutils_esrgan",
        feature = "endpoints_fal-ai_imageutils_marigold-depth",
        feature = "endpoints_fal-ai_imageutils_nsfw",
        feature = "endpoints_fal-ai_imageutils_rembg",
        feature = "endpoints_fal-ai_imageutils_sam"
    )))
)]
pub mod imageutils;
#[cfg(any(feature = "endpoints_fal-ai_inpaint"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_inpaint"))))]
pub mod inpaint;
#[cfg(any(feature = "endpoints_fal-ai_ip-adapter-face-id"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_ip-adapter-face-id"))))]
pub mod ip_adapter_face_id;
#[cfg(any(feature = "endpoints_fal-ai_janus"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_janus"))))]
pub mod janus;
#[cfg(any(
    feature = "endpoints_fal-ai_kling",
    feature = "endpoints_fal-ai_kling_v1-5",
    feature = "endpoints_fal-ai_kling_v1-5_kolors-virtual-try-on"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_kling",
        feature = "endpoints_fal-ai_kling_v1-5",
        feature = "endpoints_fal-ai_kling_v1-5_kolors-virtual-try-on"
    )))
)]
pub mod kling;
#[cfg(any(
    feature = "endpoints_fal-ai_kling-video",
    feature = "endpoints_fal-ai_kling-video_v1",
    feature = "endpoints_fal-ai_kling-video_v1-5",
    feature = "endpoints_fal-ai_kling-video_v1-5_pro",
    feature = "endpoints_fal-ai_kling-video_v1-5_pro_effects",
    feature = "endpoints_fal-ai_kling-video_v1-5_pro_image-to-video",
    feature = "endpoints_fal-ai_kling-video_v1-5_pro_text-to-video",
    feature = "endpoints_fal-ai_kling-video_v1-5_standard",
    feature = "endpoints_fal-ai_kling-video_v1-5_standard_effects",
    feature = "endpoints_fal-ai_kling-video_v1-6",
    feature = "endpoints_fal-ai_kling-video_v1-6_pro",
    feature = "endpoints_fal-ai_kling-video_v1-6_pro_effects",
    feature = "endpoints_fal-ai_kling-video_v1-6_pro_image-to-video",
    feature = "endpoints_fal-ai_kling-video_v1-6_pro_text-to-video",
    feature = "endpoints_fal-ai_kling-video_v1-6_standard",
    feature = "endpoints_fal-ai_kling-video_v1-6_standard_effects",
    feature = "endpoints_fal-ai_kling-video_v1-6_standard_image-to-video",
    feature = "endpoints_fal-ai_kling-video_v1-6_standard_text-to-video",
    feature = "endpoints_fal-ai_kling-video_v1_pro",
    feature = "endpoints_fal-ai_kling-video_v1_pro_effects",
    feature = "endpoints_fal-ai_kling-video_v1_pro_image-to-video",
    feature = "endpoints_fal-ai_kling-video_v1_pro_text-to-video",
    feature = "endpoints_fal-ai_kling-video_v1_standard",
    feature = "endpoints_fal-ai_kling-video_v1_standard_effects",
    feature = "endpoints_fal-ai_kling-video_v1_standard_image-to-video",
    feature = "endpoints_fal-ai_kling-video_v1_standard_text-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_kling-video",
        feature = "endpoints_fal-ai_kling-video_v1",
        feature = "endpoints_fal-ai_kling-video_v1-5",
        feature = "endpoints_fal-ai_kling-video_v1-5_pro",
        feature = "endpoints_fal-ai_kling-video_v1-5_pro_effects",
        feature = "endpoints_fal-ai_kling-video_v1-5_pro_image-to-video",
        feature = "endpoints_fal-ai_kling-video_v1-5_pro_text-to-video",
        feature = "endpoints_fal-ai_kling-video_v1-5_standard",
        feature = "endpoints_fal-ai_kling-video_v1-5_standard_effects",
        feature = "endpoints_fal-ai_kling-video_v1-6",
        feature = "endpoints_fal-ai_kling-video_v1-6_pro",
        feature = "endpoints_fal-ai_kling-video_v1-6_pro_effects",
        feature = "endpoints_fal-ai_kling-video_v1-6_pro_image-to-video",
        feature = "endpoints_fal-ai_kling-video_v1-6_pro_text-to-video",
        feature = "endpoints_fal-ai_kling-video_v1-6_standard",
        feature = "endpoints_fal-ai_kling-video_v1-6_standard_effects",
        feature = "endpoints_fal-ai_kling-video_v1-6_standard_image-to-video",
        feature = "endpoints_fal-ai_kling-video_v1-6_standard_text-to-video",
        feature = "endpoints_fal-ai_kling-video_v1_pro",
        feature = "endpoints_fal-ai_kling-video_v1_pro_effects",
        feature = "endpoints_fal-ai_kling-video_v1_pro_image-to-video",
        feature = "endpoints_fal-ai_kling-video_v1_pro_text-to-video",
        feature = "endpoints_fal-ai_kling-video_v1_standard",
        feature = "endpoints_fal-ai_kling-video_v1_standard_effects",
        feature = "endpoints_fal-ai_kling-video_v1_standard_image-to-video",
        feature = "endpoints_fal-ai_kling-video_v1_standard_text-to-video"
    )))
)]
pub mod kling_video;
#[cfg(any(
    feature = "endpoints_fal-ai_kokoro",
    feature = "endpoints_fal-ai_kokoro_american-english",
    feature = "endpoints_fal-ai_kokoro_brazilian-portuguese",
    feature = "endpoints_fal-ai_kokoro_british-english",
    feature = "endpoints_fal-ai_kokoro_french",
    feature = "endpoints_fal-ai_kokoro_hindi",
    feature = "endpoints_fal-ai_kokoro_italian",
    feature = "endpoints_fal-ai_kokoro_japanese",
    feature = "endpoints_fal-ai_kokoro_mandarin-chinese",
    feature = "endpoints_fal-ai_kokoro_spanish"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_kokoro",
        feature = "endpoints_fal-ai_kokoro_american-english",
        feature = "endpoints_fal-ai_kokoro_brazilian-portuguese",
        feature = "endpoints_fal-ai_kokoro_british-english",
        feature = "endpoints_fal-ai_kokoro_french",
        feature = "endpoints_fal-ai_kokoro_hindi",
        feature = "endpoints_fal-ai_kokoro_italian",
        feature = "endpoints_fal-ai_kokoro_japanese",
        feature = "endpoints_fal-ai_kokoro_mandarin-chinese",
        feature = "endpoints_fal-ai_kokoro_spanish"
    )))
)]
pub mod kokoro;
#[cfg(any(
    feature = "endpoints_fal-ai_kolors",
    feature = "endpoints_fal-ai_kolors_image-to-image"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_kolors",
        feature = "endpoints_fal-ai_kolors_image-to-image"
    )))
)]
pub mod kolors;
#[cfg(any(feature = "endpoints_fal-ai_latentsync"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_latentsync"))))]
pub mod latentsync;
#[cfg(any(feature = "endpoints_fal-ai_layer-diffusion"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_layer-diffusion"))))]
pub mod layer_diffusion;
#[cfg(any(feature = "endpoints_fal-ai_lcm"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_lcm"))))]
pub mod lcm;
#[cfg(any(feature = "endpoints_fal-ai_lcm-sd15-i2i"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_lcm-sd15-i2i"))))]
pub mod lcm_sd15_i2i;
#[cfg(any(
    feature = "endpoints_fal-ai_leffa",
    feature = "endpoints_fal-ai_leffa_pose-transfer",
    feature = "endpoints_fal-ai_leffa_virtual-tryon"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_leffa",
        feature = "endpoints_fal-ai_leffa_pose-transfer",
        feature = "endpoints_fal-ai_leffa_virtual-tryon"
    )))
)]
pub mod leffa;
#[cfg(any(
    feature = "endpoints_fal-ai_lightning-models",
    feature = "endpoints_fal-ai_lightning-models_image-to-image",
    feature = "endpoints_fal-ai_lightning-models_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_lightning-models",
        feature = "endpoints_fal-ai_lightning-models_image-to-image",
        feature = "endpoints_fal-ai_lightning-models_inpainting"
    )))
)]
pub mod lightning_models;
#[cfg(any(
    feature = "endpoints_fal-ai_live-portrait",
    feature = "endpoints_fal-ai_live-portrait_image",
    feature = "endpoints_fal-ai_live-portrait_video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_live-portrait",
        feature = "endpoints_fal-ai_live-portrait_image",
        feature = "endpoints_fal-ai_live-portrait_video"
    )))
)]
pub mod live_portrait;
#[cfg(any(feature = "endpoints_fal-ai_llava-next"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_llava-next"))))]
pub mod llava_next;
#[cfg(any(feature = "endpoints_fal-ai_llavav15-13b"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_llavav15-13b"))))]
pub mod llavav15_13b;
#[cfg(any(
    feature = "endpoints_fal-ai_lora",
    feature = "endpoints_fal-ai_lora_image-to-image",
    feature = "endpoints_fal-ai_lora_inpaint"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_lora",
        feature = "endpoints_fal-ai_lora_image-to-image",
        feature = "endpoints_fal-ai_lora_inpaint"
    )))
)]
pub mod lora;
#[cfg(any(
    feature = "endpoints_fal-ai_ltx-video",
    feature = "endpoints_fal-ai_ltx-video_image-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_ltx-video",
        feature = "endpoints_fal-ai_ltx-video_image-to-video"
    )))
)]
pub mod ltx_video;
#[cfg(any(
    feature = "endpoints_fal-ai_ltx-video-v095",
    feature = "endpoints_fal-ai_ltx-video-v095_extend",
    feature = "endpoints_fal-ai_ltx-video-v095_image-to-video",
    feature = "endpoints_fal-ai_ltx-video-v095_multiconditioning"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_ltx-video-v095",
        feature = "endpoints_fal-ai_ltx-video-v095_extend",
        feature = "endpoints_fal-ai_ltx-video-v095_image-to-video",
        feature = "endpoints_fal-ai_ltx-video-v095_multiconditioning"
    )))
)]
pub mod ltx_video_v095;
#[cfg(any(
    feature = "endpoints_fal-ai_luma-dream-machine",
    feature = "endpoints_fal-ai_luma-dream-machine_image-to-video",
    feature = "endpoints_fal-ai_luma-dream-machine_ray-2",
    feature = "endpoints_fal-ai_luma-dream-machine_ray-2_image-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_luma-dream-machine",
        feature = "endpoints_fal-ai_luma-dream-machine_image-to-video",
        feature = "endpoints_fal-ai_luma-dream-machine_ray-2",
        feature = "endpoints_fal-ai_luma-dream-machine_ray-2_image-to-video"
    )))
)]
pub mod luma_dream_machine;
#[cfg(any(
    feature = "endpoints_fal-ai_luma-photon",
    feature = "endpoints_fal-ai_luma-photon_flash"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_luma-photon",
        feature = "endpoints_fal-ai_luma-photon_flash"
    )))
)]
pub mod luma_photon;
#[cfg(any(
    feature = "endpoints_fal-ai_lumina-image",
    feature = "endpoints_fal-ai_lumina-image_v2"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_lumina-image",
        feature = "endpoints_fal-ai_lumina-image_v2"
    )))
)]
pub mod lumina_image;
#[cfg(any(
    feature = "endpoints_fal-ai_mini-cpm",
    feature = "endpoints_fal-ai_mini-cpm_video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_mini-cpm",
        feature = "endpoints_fal-ai_mini-cpm_video"
    )))
)]
pub mod mini_cpm;
#[cfg(any(
    feature = "endpoints_fal-ai_minimax",
    feature = "endpoints_fal-ai_minimax_image-to-video",
    feature = "endpoints_fal-ai_minimax_video-01",
    feature = "endpoints_fal-ai_minimax_video-01-director",
    feature = "endpoints_fal-ai_minimax_video-01-director_image-to-video",
    feature = "endpoints_fal-ai_minimax_video-01-live",
    feature = "endpoints_fal-ai_minimax_video-01-live_image-to-video",
    feature = "endpoints_fal-ai_minimax_video-01-subject-reference",
    feature = "endpoints_fal-ai_minimax_video-01_image-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_minimax",
        feature = "endpoints_fal-ai_minimax_image-to-video",
        feature = "endpoints_fal-ai_minimax_video-01",
        feature = "endpoints_fal-ai_minimax_video-01-director",
        feature = "endpoints_fal-ai_minimax_video-01-director_image-to-video",
        feature = "endpoints_fal-ai_minimax_video-01-live",
        feature = "endpoints_fal-ai_minimax_video-01-live_image-to-video",
        feature = "endpoints_fal-ai_minimax_video-01-subject-reference",
        feature = "endpoints_fal-ai_minimax_video-01_image-to-video"
    )))
)]
pub mod minimax;
#[cfg(any(feature = "endpoints_fal-ai_minimax-image"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_minimax-image"))))]
pub mod minimax_image;
#[cfg(any(feature = "endpoints_fal-ai_minimax-music"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_minimax-music"))))]
pub mod minimax_music;
#[cfg(any(
    feature = "endpoints_fal-ai_mmaudio-v2",
    feature = "endpoints_fal-ai_mmaudio-v2_text-to-audio"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_mmaudio-v2",
        feature = "endpoints_fal-ai_mmaudio-v2_text-to-audio"
    )))
)]
pub mod mmaudio_v2;
#[cfg(any(feature = "endpoints_fal-ai_mochi-v1"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_mochi-v1"))))]
pub mod mochi_v1;
#[cfg(any(
    feature = "endpoints_fal-ai_moondream",
    feature = "endpoints_fal-ai_moondream_batched"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_moondream",
        feature = "endpoints_fal-ai_moondream_batched"
    )))
)]
pub mod moondream;
#[cfg(any(
    feature = "endpoints_fal-ai_moondream-next",
    feature = "endpoints_fal-ai_moondream-next_batch",
    feature = "endpoints_fal-ai_moondream-next_detection"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_moondream-next",
        feature = "endpoints_fal-ai_moondream-next_batch",
        feature = "endpoints_fal-ai_moondream-next_detection"
    )))
)]
pub mod moondream_next;
#[cfg(any(feature = "endpoints_fal-ai_musetalk"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_musetalk"))))]
pub mod musetalk;
#[cfg(any(
    feature = "endpoints_fal-ai_nafnet",
    feature = "endpoints_fal-ai_nafnet_deblur",
    feature = "endpoints_fal-ai_nafnet_denoise"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_nafnet",
        feature = "endpoints_fal-ai_nafnet_deblur",
        feature = "endpoints_fal-ai_nafnet_denoise"
    )))
)]
pub mod nafnet;
#[cfg(any(feature = "endpoints_fal-ai_omni-zero"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_omni-zero"))))]
pub mod omni_zero;
#[cfg(any(feature = "endpoints_fal-ai_omnigen-v1"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_omnigen-v1"))))]
pub mod omnigen_v1;
#[cfg(any(feature = "endpoints_fal-ai_photomaker"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_photomaker"))))]
pub mod photomaker;
#[cfg(any(feature = "endpoints_fal-ai_pixart-sigma"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_pixart-sigma"))))]
pub mod pixart_sigma;
#[cfg(any(
    feature = "endpoints_fal-ai_pixverse",
    feature = "endpoints_fal-ai_pixverse_v3-5",
    feature = "endpoints_fal-ai_pixverse_v3-5_image-to-video",
    feature = "endpoints_fal-ai_pixverse_v3-5_image-to-video_fast",
    feature = "endpoints_fal-ai_pixverse_v3-5_text-to-video",
    feature = "endpoints_fal-ai_pixverse_v3-5_text-to-video_fast"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_pixverse",
        feature = "endpoints_fal-ai_pixverse_v3-5",
        feature = "endpoints_fal-ai_pixverse_v3-5_image-to-video",
        feature = "endpoints_fal-ai_pixverse_v3-5_image-to-video_fast",
        feature = "endpoints_fal-ai_pixverse_v3-5_text-to-video",
        feature = "endpoints_fal-ai_pixverse_v3-5_text-to-video_fast"
    )))
)]
pub mod pixverse;
#[cfg(any(
    feature = "endpoints_fal-ai_playai",
    feature = "endpoints_fal-ai_playai_create-voice",
    feature = "endpoints_fal-ai_playai_train",
    feature = "endpoints_fal-ai_playai_train_dialog"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_playai",
        feature = "endpoints_fal-ai_playai_create-voice",
        feature = "endpoints_fal-ai_playai_train",
        feature = "endpoints_fal-ai_playai_train_dialog"
    )))
)]
pub mod playai;
#[cfg(any(
    feature = "endpoints_fal-ai_playground-v25",
    feature = "endpoints_fal-ai_playground-v25_image-to-image",
    feature = "endpoints_fal-ai_playground-v25_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_playground-v25",
        feature = "endpoints_fal-ai_playground-v25_image-to-image",
        feature = "endpoints_fal-ai_playground-v25_inpainting"
    )))
)]
pub mod playground_v25;
#[cfg(any(feature = "endpoints_fal-ai_post-processing"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_post-processing"))))]
pub mod post_processing;
#[cfg(any(feature = "endpoints_fal-ai_pulid"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_pulid"))))]
pub mod pulid;
#[cfg(any(
    feature = "endpoints_fal-ai_realistic-vision",
    feature = "endpoints_fal-ai_realistic-vision_image-to-image",
    feature = "endpoints_fal-ai_realistic-vision_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_realistic-vision",
        feature = "endpoints_fal-ai_realistic-vision_image-to-image",
        feature = "endpoints_fal-ai_realistic-vision_inpainting"
    )))
)]
pub mod realistic_vision;
#[cfg(any(feature = "endpoints_fal-ai_recraft-20b"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_recraft-20b"))))]
pub mod recraft_20b;
#[cfg(any(feature = "endpoints_fal-ai_recraft-clarity-upscale"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_recraft-clarity-upscale")))
)]
pub mod recraft_clarity_upscale;
#[cfg(any(feature = "endpoints_fal-ai_recraft-creative-upscale"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_recraft-creative-upscale")))
)]
pub mod recraft_creative_upscale;
#[cfg(any(
    feature = "endpoints_fal-ai_recraft-v3",
    feature = "endpoints_fal-ai_recraft-v3_create-style"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_recraft-v3",
        feature = "endpoints_fal-ai_recraft-v3_create-style"
    )))
)]
pub mod recraft_v3;
#[cfg(any(feature = "endpoints_fal-ai_retoucher"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_retoucher"))))]
pub mod retoucher;
#[cfg(any(
    feature = "endpoints_fal-ai_sa2va",
    feature = "endpoints_fal-ai_sa2va_v4b",
    feature = "endpoints_fal-ai_sa2va_v4b_image",
    feature = "endpoints_fal-ai_sa2va_v4b_video",
    feature = "endpoints_fal-ai_sa2va_v8b",
    feature = "endpoints_fal-ai_sa2va_v8b_image",
    feature = "endpoints_fal-ai_sa2va_v8b_video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_sa2va",
        feature = "endpoints_fal-ai_sa2va_v4b",
        feature = "endpoints_fal-ai_sa2va_v4b_image",
        feature = "endpoints_fal-ai_sa2va_v4b_video",
        feature = "endpoints_fal-ai_sa2va_v8b",
        feature = "endpoints_fal-ai_sa2va_v8b_image",
        feature = "endpoints_fal-ai_sa2va_v8b_video"
    )))
)]
pub mod sa2va;
#[cfg(any(
    feature = "endpoints_fal-ai_sadtalker",
    feature = "endpoints_fal-ai_sadtalker_reference"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_sadtalker",
        feature = "endpoints_fal-ai_sadtalker_reference"
    )))
)]
pub mod sadtalker;
#[cfg(any(
    feature = "endpoints_fal-ai_sam2",
    feature = "endpoints_fal-ai_sam2_auto-segment",
    feature = "endpoints_fal-ai_sam2_image",
    feature = "endpoints_fal-ai_sam2_image-rle"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_sam2",
        feature = "endpoints_fal-ai_sam2_auto-segment",
        feature = "endpoints_fal-ai_sam2_image",
        feature = "endpoints_fal-ai_sam2_image-rle"
    )))
)]
pub mod sam2;
#[cfg(any(feature = "endpoints_fal-ai_sana"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_sana"))))]
pub mod sana;
#[cfg(any(
    feature = "endpoints_fal-ai_sd15-depth-controlnet",
    feature = "endpoints_fal-ai_sd15-depth-controlnet_image-to-image",
    feature = "endpoints_fal-ai_sd15-depth-controlnet_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_sd15-depth-controlnet",
        feature = "endpoints_fal-ai_sd15-depth-controlnet_image-to-image",
        feature = "endpoints_fal-ai_sd15-depth-controlnet_inpainting"
    )))
)]
pub mod sd15_depth_controlnet;
#[cfg(any(
    feature = "endpoints_fal-ai_sdxl-controlnet-union",
    feature = "endpoints_fal-ai_sdxl-controlnet-union_image-to-image",
    feature = "endpoints_fal-ai_sdxl-controlnet-union_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_sdxl-controlnet-union",
        feature = "endpoints_fal-ai_sdxl-controlnet-union_image-to-image",
        feature = "endpoints_fal-ai_sdxl-controlnet-union_inpainting"
    )))
)]
pub mod sdxl_controlnet_union;
#[cfg(any(feature = "endpoints_fal-ai_skyreels-i2v"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_skyreels-i2v"))))]
pub mod skyreels_i2v;
#[cfg(any(feature = "endpoints_fal-ai_stable-audio"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_stable-audio"))))]
pub mod stable_audio;
#[cfg(any(
    feature = "endpoints_fal-ai_stable-cascade",
    feature = "endpoints_fal-ai_stable-cascade_sote-diffusion"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_stable-cascade",
        feature = "endpoints_fal-ai_stable-cascade_sote-diffusion"
    )))
)]
pub mod stable_cascade;
#[cfg(any(
    feature = "endpoints_fal-ai_stable-diffusion-v15",
    feature = "endpoints_fal-ai_stable-diffusion-v15_image-to-image",
    feature = "endpoints_fal-ai_stable-diffusion-v15_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_stable-diffusion-v15",
        feature = "endpoints_fal-ai_stable-diffusion-v15_image-to-image",
        feature = "endpoints_fal-ai_stable-diffusion-v15_inpainting"
    )))
)]
pub mod stable_diffusion_v15;
#[cfg(any(
    feature = "endpoints_fal-ai_stable-diffusion-v35-large",
    feature = "endpoints_fal-ai_stable-diffusion-v35-large_image-to-image",
    feature = "endpoints_fal-ai_stable-diffusion-v35-large_inpaint",
    feature = "endpoints_fal-ai_stable-diffusion-v35-large_turbo",
    feature = "endpoints_fal-ai_stable-diffusion-v35-large_turbo_image-to-image",
    feature = "endpoints_fal-ai_stable-diffusion-v35-large_turbo_inpaint"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_stable-diffusion-v35-large",
        feature = "endpoints_fal-ai_stable-diffusion-v35-large_image-to-image",
        feature = "endpoints_fal-ai_stable-diffusion-v35-large_inpaint",
        feature = "endpoints_fal-ai_stable-diffusion-v35-large_turbo",
        feature = "endpoints_fal-ai_stable-diffusion-v35-large_turbo_image-to-image",
        feature = "endpoints_fal-ai_stable-diffusion-v35-large_turbo_inpaint"
    )))
)]
pub mod stable_diffusion_v35_large;
#[cfg(any(feature = "endpoints_fal-ai_stable-diffusion-v35-medium"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_stable-diffusion-v35-medium")))
)]
pub mod stable_diffusion_v35_medium;
#[cfg(any(
    feature = "endpoints_fal-ai_stable-diffusion-v3-medium",
    feature = "endpoints_fal-ai_stable-diffusion-v3-medium_image-to-image"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_stable-diffusion-v3-medium",
        feature = "endpoints_fal-ai_stable-diffusion-v3-medium_image-to-image"
    )))
)]
pub mod stable_diffusion_v3_medium;
#[cfg(any(
    feature = "endpoints_fal-ai_stable-video",
    feature = "endpoints_fal-ai_stable-video_text-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_stable-video",
        feature = "endpoints_fal-ai_stable-video_text-to-video"
    )))
)]
pub mod stable_video;
#[cfg(any(feature = "endpoints_fal-ai_stepfun-video"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_stepfun-video"))))]
pub mod stepfun_video;
#[cfg(any(feature = "endpoints_fal-ai_swin2sr"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_swin2sr"))))]
pub mod swin2sr;
#[cfg(any(
    feature = "endpoints_fal-ai_switti",
    feature = "endpoints_fal-ai_switti_v512"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_switti",
        feature = "endpoints_fal-ai_switti_v512"
    )))
)]
pub mod switti;
#[cfg(any(feature = "endpoints_fal-ai_sync-lipsync"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_sync-lipsync"))))]
pub mod sync_lipsync;
#[cfg(any(feature = "endpoints_fal-ai_t2v-turbo"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_t2v-turbo"))))]
pub mod t2v_turbo;
#[cfg(any(feature = "endpoints_fal-ai_transpixar"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_transpixar"))))]
pub mod transpixar;
#[cfg(any(feature = "endpoints_fal-ai_trellis"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_trellis"))))]
pub mod trellis;
#[cfg(any(
    feature = "endpoints_fal-ai_triposr",
    feature = "endpoints_fal-ai_triposr_remeshing"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_triposr",
        feature = "endpoints_fal-ai_triposr_remeshing"
    )))
)]
pub mod triposr;
#[cfg(any(
    feature = "endpoints_fal-ai_veo2",
    feature = "endpoints_fal-ai_veo2_image-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_veo2",
        feature = "endpoints_fal-ai_veo2_image-to-video"
    )))
)]
pub mod veo2;
#[cfg(any(feature = "endpoints_fal-ai_video-prompt-generator"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_video-prompt-generator")))
)]
pub mod video_prompt_generator;
#[cfg(any(feature = "endpoints_fal-ai_video-upscaler"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_video-upscaler"))))]
pub mod video_upscaler;
#[cfg(any(
    feature = "endpoints_fal-ai_wan",
    feature = "endpoints_fal-ai_wan_v2-1",
    feature = "endpoints_fal-ai_wan_v2-1_v1-3b",
    feature = "endpoints_fal-ai_wan_v2-1_v1-3b_text-to-video"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_wan",
        feature = "endpoints_fal-ai_wan_v2-1",
        feature = "endpoints_fal-ai_wan_v2-1_v1-3b",
        feature = "endpoints_fal-ai_wan_v2-1_v1-3b_text-to-video"
    )))
)]
pub mod wan;
#[cfg(any(feature = "endpoints_fal-ai_wan-i2v"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_wan-i2v"))))]
pub mod wan_i2v;
#[cfg(any(feature = "endpoints_fal-ai_wan-t2v"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_wan-t2v"))))]
pub mod wan_t2v;
#[cfg(any(feature = "endpoints_fal-ai_whisper"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_whisper"))))]
pub mod whisper;
#[cfg(any(feature = "endpoints_fal-ai_wizper"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_wizper"))))]
pub mod wizper;
#[cfg(any(
    feature = "endpoints_fal-ai_workflowutils",
    feature = "endpoints_fal-ai_workflowutils_blur-mask",
    feature = "endpoints_fal-ai_workflowutils_canny",
    feature = "endpoints_fal-ai_workflowutils_compare-text",
    feature = "endpoints_fal-ai_workflowutils_composite-image",
    feature = "endpoints_fal-ai_workflowutils_grow-mask",
    feature = "endpoints_fal-ai_workflowutils_image-size",
    feature = "endpoints_fal-ai_workflowutils_insert-text",
    feature = "endpoints_fal-ai_workflowutils_insightface",
    feature = "endpoints_fal-ai_workflowutils_invert-mask",
    feature = "endpoints_fal-ai_workflowutils_regex-replace",
    feature = "endpoints_fal-ai_workflowutils_resize-image",
    feature = "endpoints_fal-ai_workflowutils_resize-to-max-pixels",
    feature = "endpoints_fal-ai_workflowutils_rgba-to-rgb",
    feature = "endpoints_fal-ai_workflowutils_shrink-mask",
    feature = "endpoints_fal-ai_workflowutils_teed",
    feature = "endpoints_fal-ai_workflowutils_transparent-image-to-mask"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_workflowutils",
        feature = "endpoints_fal-ai_workflowutils_blur-mask",
        feature = "endpoints_fal-ai_workflowutils_canny",
        feature = "endpoints_fal-ai_workflowutils_compare-text",
        feature = "endpoints_fal-ai_workflowutils_composite-image",
        feature = "endpoints_fal-ai_workflowutils_grow-mask",
        feature = "endpoints_fal-ai_workflowutils_image-size",
        feature = "endpoints_fal-ai_workflowutils_insert-text",
        feature = "endpoints_fal-ai_workflowutils_insightface",
        feature = "endpoints_fal-ai_workflowutils_invert-mask",
        feature = "endpoints_fal-ai_workflowutils_regex-replace",
        feature = "endpoints_fal-ai_workflowutils_resize-image",
        feature = "endpoints_fal-ai_workflowutils_resize-to-max-pixels",
        feature = "endpoints_fal-ai_workflowutils_rgba-to-rgb",
        feature = "endpoints_fal-ai_workflowutils_shrink-mask",
        feature = "endpoints_fal-ai_workflowutils_teed",
        feature = "endpoints_fal-ai_workflowutils_transparent-image-to-mask"
    )))
)]
pub mod workflowutils;
#[cfg(any(feature = "endpoints_fal-ai_yue"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_yue"))))]
pub mod yue;
#[cfg(any(feature = "endpoints_fal-ai_zonos"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_zonos"))))]
pub mod zonos;
