#[cfg(any(
    feature = "endpoints_rundiffusion-fal_juggernaut-flux",
    feature = "endpoints_rundiffusion-fal_juggernaut-flux_base",
    feature = "endpoints_rundiffusion-fal_juggernaut-flux_base_image-to-image",
    feature = "endpoints_rundiffusion-fal_juggernaut-flux_base_redux",
    feature = "endpoints_rundiffusion-fal_juggernaut-flux_lightning",
    feature = "endpoints_rundiffusion-fal_juggernaut-flux_lightning_redux",
    feature = "endpoints_rundiffusion-fal_juggernaut-flux_pro",
    feature = "endpoints_rundiffusion-fal_juggernaut-flux_pro_image-to-image",
    feature = "endpoints_rundiffusion-fal_juggernaut-flux_pro_redux"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_rundiffusion-fal_juggernaut-flux",
        feature = "endpoints_rundiffusion-fal_juggernaut-flux_base",
        feature = "endpoints_rundiffusion-fal_juggernaut-flux_base_image-to-image",
        feature = "endpoints_rundiffusion-fal_juggernaut-flux_base_redux",
        feature = "endpoints_rundiffusion-fal_juggernaut-flux_lightning",
        feature = "endpoints_rundiffusion-fal_juggernaut-flux_lightning_redux",
        feature = "endpoints_rundiffusion-fal_juggernaut-flux_pro",
        feature = "endpoints_rundiffusion-fal_juggernaut-flux_pro_image-to-image",
        feature = "endpoints_rundiffusion-fal_juggernaut-flux_pro_redux"
    )))
)]
pub mod juggernaut_flux;
#[cfg(any(
    feature = "endpoints_rundiffusion-fal_juggernaut-flux-lora",
    feature = "endpoints_rundiffusion-fal_juggernaut-flux-lora_image-to-image",
    feature = "endpoints_rundiffusion-fal_juggernaut-flux-lora_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_rundiffusion-fal_juggernaut-flux-lora",
        feature = "endpoints_rundiffusion-fal_juggernaut-flux-lora_image-to-image",
        feature = "endpoints_rundiffusion-fal_juggernaut-flux-lora_inpainting"
    )))
)]
pub mod juggernaut_flux_lora;
#[cfg(any(
    feature = "endpoints_rundiffusion-fal_rundiffusion-photo-flux",
    feature = "endpoints_rundiffusion-fal_rundiffusion-photo-flux_image-to-image",
    feature = "endpoints_rundiffusion-fal_rundiffusion-photo-flux_inpainting",
    feature = "endpoints_rundiffusion-fal_rundiffusion-photo-flux_juggernaut",
    feature = "endpoints_rundiffusion-fal_rundiffusion-photo-flux_juggernaut_image-to-image",
    feature = "endpoints_rundiffusion-fal_rundiffusion-photo-flux_juggernaut_inpainting"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_rundiffusion-fal_rundiffusion-photo-flux",
        feature = "endpoints_rundiffusion-fal_rundiffusion-photo-flux_image-to-image",
        feature = "endpoints_rundiffusion-fal_rundiffusion-photo-flux_inpainting",
        feature = "endpoints_rundiffusion-fal_rundiffusion-photo-flux_juggernaut",
        feature = "endpoints_rundiffusion-fal_rundiffusion-photo-flux_juggernaut_image-to-image",
        feature = "endpoints_rundiffusion-fal_rundiffusion-photo-flux_juggernaut_inpainting"
    )))
)]
pub mod rundiffusion_photo_flux;
