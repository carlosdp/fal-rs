#[cfg(any(feature = "endpoints", feature = "endpoints_fal-ai"))]
pub mod fal_ai;
#[cfg(any(feature = "endpoints", feature = "endpoints_fashn"))]
pub mod fashn;
#[cfg(any(feature = "endpoints", feature = "endpoints_rundiffusion-fal"))]
pub mod rundiffusion_fal;

mod types;
pub use types::*;
