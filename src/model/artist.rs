use crate::model::album::Album;
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

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SimiArtistsResp {
    pub code: usize,
    pub artists: Vec<Artist>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistAlbumResp {
    pub code: usize,
    #[allow(unused)]
    pub more: bool,
    pub hot_albums: Vec<Album>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ArtistBlock {
    Tracks,
    Albums,
    SimiArtists,
    #[allow(unused)]
    Empty,
}

#[derive(Clone)]
pub struct ArtistDetail {
    pub artist_name: String,
    pub tracks: Vec<Track>,
    pub albums: Vec<Album>,
    pub simi_artists: Vec<Artist>,
    pub selected_album_index: usize,
    pub selected_simi_artist_index: usize,
    pub selected_track_index: usize,
    pub artist_detail_selected_block: ArtistBlock,
    pub artist_detail_hover_block: ArtistBlock,
}
