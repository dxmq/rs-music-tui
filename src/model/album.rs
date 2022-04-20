use crate::model::artist::SimplifiedArtist;
use crate::model::enums::{AlbumType, Type};
use crate::model::image::Image;
use crate::model::page::Page;
use crate::model::track::SimplifiedTrack;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 限制
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Restrictions {
    pub reason: String,
}

// 简单专辑
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplifiedAlbum {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_group: Option<String>,
    pub album_type: Option<String>,
    pub artists: Vec<SimplifiedArtist>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub available_markets: Vec<String>,
    pub external_urls: HashMap<String, String>,
    pub href: Option<String>,
    pub id: Option<String>,
    pub images: Vec<Image>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date_precision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restrictions>,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullAlbum {
    pub artists: Vec<SimplifiedArtist>,
    pub album_type: AlbumType,
    pub available_markets: Vec<String>,
    pub copyrights: Vec<HashMap<String, String>>,
    pub external_ids: HashMap<String, String>,
    pub external_urls: HashMap<String, String>,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: u32,
    pub release_date: String,
    pub release_date_precision: String,
    pub tracks: Page<SimplifiedTrack>,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

// 所有的专辑
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullAlbums {
    pub albums: Vec<FullAlbum>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageSimpledAlbums {
    pub albums: Page<SimplifiedAlbum>,
}
