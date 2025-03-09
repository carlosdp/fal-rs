#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PhotoMakerOutput {
        pub images: Vec<Image>,
pub seed: i64
    }
    

                /// PhotoMaker
/// 
/// Customizing Realistic Human Photos via Stacked ID Embedding
/// 
/// Category: image-to-image
/// Machine Type: A6000
                pub fn photomaker(params: PhotoMakerInput) -> FalRequest<PhotoMakerInput, PhotoMakerOutput> {
                    FalRequest::new(
                        "fal-ai/photomaker",
                        params
                    )
                }
                