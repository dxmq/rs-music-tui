use crate::model::enums::Type;
use crate::model::image::Image;
use crate::model::user::PublicUser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplifiedPlaylist {
    pub collaborative: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: PublicUser,
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: HashMap<String, Value>,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}
