use serde::{Deserialize, Serialize};

use crate::model::enums::{CurrentlyPlayingType, PlayingItem, RepeatState};
use crate::model::track::Track;

// 当前回放上下文
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CurrentlyPlaybackContext {
    // 播放状态
    pub is_playing: bool,
    pub timestamp: u64,
    // 当前播放的是什么
    pub currently_playing_type: CurrentlyPlayingType,
    // 重复状态
    pub repeat_state: RepeatState,
    // 当前播放项
    pub item: Option<PlayingItem>,
}

impl CurrentlyPlaybackContext {
    pub fn new(item: Option<PlayingItem>) -> Self {
        CurrentlyPlaybackContext {
            item,
            ..CurrentlyPlaybackContext::default()
        }
    }
}

impl Default for CurrentlyPlaybackContext {
    fn default() -> Self {
        CurrentlyPlaybackContext {
            is_playing: false,
            timestamp: 0,
            currently_playing_type: CurrentlyPlayingType::Track,
            repeat_state: RepeatState::Off,
            item: Some(PlayingItem::Track(Track {
                id: 0,
                name: "".to_string(),
                artists: vec![],
                album: Default::default(),
                duration: 0,
                fee: 0,
                pop: 0.0,
            })),
        }
    }
}

// 对话框
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DialogContext {
    #[allow(unused)]
    Playlist,
    #[allow(unused)]
    SubPlaylist,
    #[allow(unused)]
    PlaylistSearch,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TrackTableContext {
    MyPlaylists,
    #[allow(unused)]
    AlbumSearch,
    #[allow(unused)]
    PlaylistSearch,
    #[allow(unused)]
    SavedTracks,
    #[allow(unused)]
    RecommendedTracks,
    #[allow(unused)]
    RecentlyPlayed,
    #[allow(unused)]
    MadeForYou,
    SearchResult,
    ArtistDetail,
    AlbumDetail,
}
