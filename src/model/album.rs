use crate::model::artist::Artist;
use crate::model::table::TableItem;
use crate::model::track::Track;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: usize,
    pub name: Option<String>,
    #[serde(default)]
    pub artist: Artist,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumResp {
    pub code: usize,
    pub songs: Vec<Track>,
    pub album: Album,
}

#[derive(Clone)]
pub struct AlbumDetail {
    pub album: Album,
    pub tracks: Vec<Track>,
    pub selected_track_index: usize,
}

pub struct AlbumUi {
    pub(crate) selected_index: usize,
    pub(crate) items: Vec<TableItem>,
    pub(crate) title: String,
}
