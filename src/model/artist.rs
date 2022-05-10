use crate::model::track::Track;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: usize,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistSublistResp {
    pub code: usize,
    #[serde(default)]
    pub data: Vec<Artist>,
    #[serde(default)]
    pub has_more: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ArtistTracksResp {
    pub code: usize,
    #[serde(rename = "songs")]
    pub tracks: Vec<Track>,
}
