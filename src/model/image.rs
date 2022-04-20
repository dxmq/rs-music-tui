use serde::{Deserialize, Serialize};

// 专辑封面
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    pub height: Option<u32>,
    pub url: String,
    pub width: Option<u32>,
}
