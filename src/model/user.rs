use crate::model::enums::Type;
use crate::model::image::Image;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicUser {
    pub display_name: Option<String>,
    pub external_urls: HashMap<String, String>,
    pub followers: Option<HashMap<String, Option<Value>>>,
    pub href: String,
    pub id: String,
    pub images: Option<Vec<Image>>,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}
