use crate::model::album::{Restrictions, SimplifiedAlbum};
use crate::model::artist::SimplifiedArtist;
use crate::model::enums::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackLink {
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

/// 完整的歌曲
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullTrack {
    pub album: SimplifiedAlbum,
    pub artists: Vec<SimplifiedArtist>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub available_markets: Vec<String>,
    pub disc_number: i32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_ids: HashMap<String, String>,
    pub external_urls: HashMap<String, String>,
    pub href: Option<String>,
    pub id: Option<String>,
    pub is_local: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_playable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_from: Option<TrackLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restrictions>,
    pub name: String,
    pub popularity: u32,
    pub preview_url: Option<String>,
    pub track_number: u32,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplifiedTrack {
    pub artists: Vec<SimplifiedArtist>,
    pub available_markets: Option<Vec<String>>,
    pub disc_number: i32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    #[serde(default)]
    pub href: Option<String>,
    pub id: Option<String>,
    pub is_local: bool,
    pub name: String,
    pub preview_url: Option<String>,
    pub track_number: u32,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}
