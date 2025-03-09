#[allow(unused_imports)]
use serde::{Serialize, Deserialize};#[allow(unused_imports)]
use crate::prelude::*;


                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        pub images: Vec<File>,
/// Seed used for generation/// Seed used for generation/// 42

pub seed: i64
    }
    

                /// Imagen3
/// 
/// Imagen3 is a high-quality text-to-image model that generates realistic images from text prompts.
/// 
/// Category: text-to-image
/// 
/// 
/// 
/// Generate customized images using reference images of a subject.
/// 
/// Supports customization for:
/// - Products: Generate new images of products in different contexts
/// - People: Create variations of people in different poses and settings
/// - Animal companions: Generate new images of pets in different scenes
/// 
/// The reference images guide the model to maintain key characteristics
/// of the subject while generating new images matching your prompt.
                pub fn subject_reference(params: SubjectCustomizeInput) -> FalRequest<SubjectCustomizeInput, Output> {
                    FalRequest::new(
                        "fal-ai/imagen3",
                        params
                    )
                }
                
                
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        pub images: Vec<File>,
/// Seed used for generation/// Seed used for generation/// 42

pub seed: i64
    }
    

                /// Imagen3 Fast
/// 
/// Imagen3 Fast is a high-quality text-to-image model that generates realistic images from text prompts.
/// 
/// Category: text-to-image
/// 
/// 
/// 
/// Generate customized images using reference images of a subject.
/// 
/// Supports customization for:
/// - Products: Generate new images of products in different contexts
/// - People: Create variations of people in different poses and settings
/// - Animal companions: Generate new images of pets in different scenes
/// 
/// The reference images guide the model to maintain key characteristics
/// of the subject while generating new images matching your prompt.
                pub fn subject_reference(params: SubjectCustomizeInput) -> FalRequest<SubjectCustomizeInput, Output> {
                    FalRequest::new(
                        "fal-ai/imagen3/fast",
                        params
                    )
                }
                