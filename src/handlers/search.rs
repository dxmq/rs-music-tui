use crate::model::album::Album;
use crate::model::artist::Artist;
use crate::model::playlist::Playlist;
use crate::model::track::Track;

#[allow(unused)]
pub enum SearchResult {
    Tracks(Vec<Track>),
    Albums(Vec<Album>),
    Artists(Vec<Artist>),
    Playlists(Vec<Playlist>),
}

pub struct SearchResults {
    pub tracks: Option<Vec<Track>>,
    pub albums: Option<Vec<Album>>,
    pub artists: Option<Vec<Artist>>,
    pub playlists: Option<Vec<Playlist>>,
    pub selected_album_index: Option<usize>,
    pub selected_artists_index: Option<usize>,
    pub selected_playlists_index: Option<usize>,
    pub selected_tracks_index: Option<usize>,
    pub selected_shows_index: Option<usize>,
    pub hovered_block: SearchResultBlock,
    pub selected_block: SearchResultBlock,
}

impl Default for SearchResults {
    fn default() -> Self {
        SearchResults {
            tracks: None,
            albums: None,
            artists: None,
            playlists: None,
            selected_album_index: None,
            selected_artists_index: None,
            selected_playlists_index: None,
            selected_tracks_index: None,
            selected_shows_index: None,
            selected_block: SearchResultBlock::Empty,
            hovered_block: SearchResultBlock::TrackSearch,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum SearchResultBlock {
    AlbumSearch,
    TrackSearch,
    ArtistSearch,
    PlaylistSearch,
    Empty,
}

impl SearchResult {
    pub fn new(search_type: SearchType) -> Self {
        match search_type {
            SearchType::Track => SearchResult::Tracks(vec![]),
            SearchType::Album => SearchResult::Albums(vec![]),
            SearchType::Artist => SearchResult::Artists(vec![]),
            SearchType::Playlist => SearchResult::Playlists(vec![]),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultResp<T> {
    pub code: usize,
    pub result: Option<T>,
}

pub type SearchTrackResp = ResultResp<SearchResultTrack>;
pub type SearchArtistResp = ResultResp<SearchResultArtist>;
pub type SearchPlaylistResp = ResultResp<SearchResultPlaylist>;
pub type SearchAlbumResp = ResultResp<SearchResultAlbum>;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SearchResultTrack {
    pub songs: Vec<Track>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultArtist {
    #[serde(default)]
    pub artists: Vec<Artist>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultPlaylist {
    #[serde(default)]
    pub playlists: Vec<Playlist>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultAlbum {
    #[serde(default)]
    pub albums: Vec<Album>,
}

#[derive(Clone, Copy)]
pub enum SearchType {
    Track,
    Album,
    Artist,
    Playlist,
}
