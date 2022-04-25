use crate::model::track::Track;
use crate::model::Id;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub id: usize,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserPlaylistResp {
    pub code: usize,
    #[serde(default)]
    pub playlist: Vec<Playlist>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistDetail {
    pub id: usize,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub tracks: Vec<Track>,
    #[serde(default)]
    pub track_ids: Vec<Id>,
    pub user_id: usize,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlaylistDetailResp {
    pub code: usize,
    pub playlist: Option<PlaylistDetail>,
}
