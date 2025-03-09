#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_playai",
    feature = "endpoints_fal-ai_playai_train",
    feature = "endpoints_fal-ai_playai_train_dialog"
))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "endpoints",
        feature = "endpoints_fal-ai",
        feature = "endpoints_fal-ai_playai",
        feature = "endpoints_fal-ai_playai_train",
        feature = "endpoints_fal-ai_playai_train_dialog"
    )))
)]
pub mod dialog;
