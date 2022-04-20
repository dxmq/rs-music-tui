use std::collections::HashMap;

use crate::model::enums::Type;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplifiedArtist {
    pub external_urls: HashMap<String, String>,
    pub href: Option<String>,
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: Option<String>,
}
