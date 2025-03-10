#[cfg(any(feature = "endpoints_fal-ai_playai_create-voice"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "endpoints_fal-ai_playai_create-voice")))
)]
pub mod create_voice;
#[cfg(any(
    feature = "endpoints_fal-ai_playai_train",
    feature = "endpoints_fal-ai_playai_train_dialog"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints_fal-ai_playai_train",
        feature = "endpoints_fal-ai_playai_train_dialog"
    )))
)]
pub mod train;
