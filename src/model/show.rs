use crate::model::image::Image;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullEpisode {
    pub audio_preview_url: Option<String>,
    pub description: String,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: bool,
    pub is_playable: bool,
    /// Note: This field is deprecated and might be removed in the future. Please use the languages field instead
    pub language: String,
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub resume_point: Option<ResumePoint>,
    pub show: SimplifiedShow,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplifiedShow {
    pub available_markets: Vec<String>,
    pub copyrights: Vec<HashMap<String, String>>,
    pub description: String,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: Option<bool>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    pub publisher: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResumePoint {
    pub fully_played: bool,
    pub resume_position_ms: u32,
}
