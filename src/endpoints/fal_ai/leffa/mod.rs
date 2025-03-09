#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_leffa",
    feature = "endpoints_fal-ai_leffa_pose-transfer"
))]
pub mod pose_transfer;
#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_leffa",
    feature = "endpoints_fal-ai_leffa_virtual-tryon"
))]
pub mod virtual_tryon;
