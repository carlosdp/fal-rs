#[cfg(any(feature = "endpoints_fal-ai_ideogram_upscale"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "endpoints_fal-ai_ideogram_upscale"))))]
pub mod upscale;
#[cfg(any(
    feature = "endpoints_fal-ai_ideogram_v2",
    feature = "endpoints_fal-ai_ideogram_v2_edit",
    feature = "endpoints_fal-ai_ideogram_v2_remix",
    feature = "endpoints_fal-ai_ideogram_v2_turbo",
    feature = "endpoints_fal-ai_ideogram_v2_turbo_edit",
    feature = "endpoints_fal-ai_ideogram_v2_turbo_remix"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_ideogram_v2",
        feature = "endpoints_fal-ai_ideogram_v2_edit",
        feature = "endpoints_fal-ai_ideogram_v2_remix",
        feature = "endpoints_fal-ai_ideogram_v2_turbo",
        feature = "endpoints_fal-ai_ideogram_v2_turbo_edit",
        feature = "endpoints_fal-ai_ideogram_v2_turbo_remix"
    )))
)]
pub mod v2;
#[cfg(any(
    feature = "endpoints_fal-ai_ideogram_v2a",
    feature = "endpoints_fal-ai_ideogram_v2a_remix",
    feature = "endpoints_fal-ai_ideogram_v2a_turbo",
    feature = "endpoints_fal-ai_ideogram_v2a_turbo_remix"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_ideogram_v2a",
        feature = "endpoints_fal-ai_ideogram_v2a_remix",
        feature = "endpoints_fal-ai_ideogram_v2a_turbo",
        feature = "endpoints_fal-ai_ideogram_v2a_turbo_remix"
    )))
)]
pub mod v2a;
