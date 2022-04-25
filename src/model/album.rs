#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: usize,
    pub name: Option<String>,
    #[serde(default)]
    pub pic_url: String,
    pub pic: usize,
}
