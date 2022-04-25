#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: usize,
    pub name: Option<String>,
}
