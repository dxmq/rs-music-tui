
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Dialog {
    pub tips: String,
    pub item_name: String,
    pub confirm: bool,
}