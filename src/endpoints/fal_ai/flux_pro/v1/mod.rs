#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux-pro",
    feature = "endpoints_fal-ai_flux-pro_v1",
    feature = "endpoints_fal-ai_flux-pro_v1_canny"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux-pro",
        feature = "endpoints_fal-ai_flux-pro_v1",
        feature = "endpoints_fal-ai_flux-pro_v1_canny"
    )))
)]
pub mod canny;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux-pro",
    feature = "endpoints_fal-ai_flux-pro_v1",
    feature = "endpoints_fal-ai_flux-pro_v1_canny-finetuned"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux-pro",
        feature = "endpoints_fal-ai_flux-pro_v1",
        feature = "endpoints_fal-ai_flux-pro_v1_canny-finetuned"
    )))
)]
pub mod canny_finetuned;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux-pro",
    feature = "endpoints_fal-ai_flux-pro_v1",
    feature = "endpoints_fal-ai_flux-pro_v1_depth"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux-pro",
        feature = "endpoints_fal-ai_flux-pro_v1",
        feature = "endpoints_fal-ai_flux-pro_v1_depth"
    )))
)]
pub mod depth;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux-pro",
    feature = "endpoints_fal-ai_flux-pro_v1",
    feature = "endpoints_fal-ai_flux-pro_v1_depth-finetuned"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux-pro",
        feature = "endpoints_fal-ai_flux-pro_v1",
        feature = "endpoints_fal-ai_flux-pro_v1_depth-finetuned"
    )))
)]
pub mod depth_finetuned;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux-pro",
    feature = "endpoints_fal-ai_flux-pro_v1",
    feature = "endpoints_fal-ai_flux-pro_v1_fill"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux-pro",
        feature = "endpoints_fal-ai_flux-pro_v1",
        feature = "endpoints_fal-ai_flux-pro_v1_fill"
    )))
)]
pub mod fill;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux-pro",
    feature = "endpoints_fal-ai_flux-pro_v1",
    feature = "endpoints_fal-ai_flux-pro_v1_fill-finetuned"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux-pro",
        feature = "endpoints_fal-ai_flux-pro_v1",
        feature = "endpoints_fal-ai_flux-pro_v1_fill-finetuned"
    )))
)]
pub mod fill_finetuned;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux-pro",
    feature = "endpoints_fal-ai_flux-pro_v1",
    feature = "endpoints_fal-ai_flux-pro_v1_outpaint"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux-pro",
        feature = "endpoints_fal-ai_flux-pro_v1",
        feature = "endpoints_fal-ai_flux-pro_v1_outpaint"
    )))
)]
pub mod outpaint;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_flux-pro",
    feature = "endpoints_fal-ai_flux-pro_v1",
    feature = "endpoints_fal-ai_flux-pro_v1_redux"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_flux-pro",
        feature = "endpoints_fal-ai_flux-pro_v1",
        feature = "endpoints_fal-ai_flux-pro_v1_redux"
    )))
)]
pub mod redux;
