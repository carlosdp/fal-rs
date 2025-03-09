use std::collections::HashMap;

use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum IpAdapterImageUrlProperty {
    #[default]
    String(String),
    Array(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum SystemPromptProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum ErrorProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HasNsfwConcepts {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub ty: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum SpeakerIdProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum AddonsProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum LoopModeProperty {
    #[default]
    #[serde(rename = "pingpong")]
    Pingpong,
    #[serde(rename = "loop")]
    Loop,
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum LanguageCodeProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum FileSizeProperty {
    #[default]
    Integer(i64),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum SeedProperty {
    #[default]
    Integer(i64),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum ScaleProperty {
    #[default]
    Object(HashMap<String, serde_json::Value>),
    Number(f64),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum FileNameProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Timings {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub ty: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum BboxConditionProperty {
    #[default]
    Array(Vec<i64>),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum DataArchiveFormatProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum DurationSecondsProperty {
    #[default]
    Number(f64),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum ContentTypeProperty {
    #[default]
    String(String),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum WidthProperty {
    #[default]
    Integer(i64),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum FillImageUrlProperty {
    #[default]
    Array(Vec<String>),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum HeightProperty {
    #[default]
    Integer(i64),
    Null(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum TopAlignProperty {
    #[default]
    String(String),
    Number(f64),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum RleProperty {
    #[default]
    String(String),
    Array(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum ImageSizeProperty {
    #[default]
    ImageSize(ImageSize),
    #[serde(rename = "square_hd")]
    SquareHd,
    #[serde(rename = "square")]
    Square,
    #[serde(rename = "portrait_4_3")]
    Portrait43,
    #[serde(rename = "portrait_16_9")]
    Portrait169,
    #[serde(rename = "landscape_4_3")]
    Landscape43,
    #[serde(rename = "landscape_16_9")]
    Landscape169,
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum AspectRatioProperty {
    #[default]
    #[serde(rename = "21:9")]
    Property_21_9,
    #[serde(rename = "16:9")]
    Property_16_9,
    #[serde(rename = "4:3")]
    Property_4_3,
    #[serde(rename = "3:2")]
    Property_3_2,
    #[serde(rename = "1:1")]
    Property_1_1,
    #[serde(rename = "2:3")]
    Property_2_3,
    #[serde(rename = "3:4")]
    Property_3_4,
    #[serde(rename = "9:16")]
    Property_9_16,
    #[serde(rename = "9:21")]
    Property_9_21,
    String(String),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum VideoSizeProperty {
    #[default]
    ImageSize(ImageSize),
    #[serde(rename = "square_hd")]
    SquareHd,
    #[serde(rename = "square")]
    Square,
    #[serde(rename = "portrait_4_3")]
    Portrait43,
    #[serde(rename = "portrait_16_9")]
    Portrait169,
    #[serde(rename = "landscape_4_3")]
    Landscape43,
    #[serde(rename = "landscape_16_9")]
    Landscape169,
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum LeftAlignProperty {
    #[default]
    String(String),
    Number(f64),
}

#[derive(Debug, Serialize, Deserialize, smart_default::SmartDefault)]
#[allow(non_camel_case_types)]
pub enum MediaProperty {
    #[default]
    Video(Video),
    Audio(Audio),
}
