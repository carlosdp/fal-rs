#[cfg(any(
    feature = "endpoints",
    feature = "endpoints_fal-ai",
    feature = "endpoints_fal-ai_hyper3d",
    feature = "endpoints_fal-ai_hyper3d_rodin"
))]
pub mod rodin;
