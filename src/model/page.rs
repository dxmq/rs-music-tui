use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Page<T> {
    pub href: String,
    pub item: Vec<T>,
    pub offset: u32,
    pub limit: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub total: u32,
}
