use crate::model::enums::{CurrentlyPlayingType, PlayingItem, RepeatState};
use serde::{Deserialize, Serialize};

// 当前回放上下文
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CurrentlyPlaybackContext {
    // 播放状态
    pub is_playing: bool,
    // 当前进度（毫秒）
    pub progress_ms: Option<u32>,
    pub timestamp: u64,
    // 当前播放的是什么
    pub currently_playing_type: CurrentlyPlayingType,
    // 重复状态
    pub repeat_state: RepeatState,
    // 当前播放项
    pub item: Option<PlayingItem>,
}

// 对话框
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DialogContext {
    #[allow(unused)]
    PlaylistWindow,
    #[allow(unused)]
    PlaylistSearch,
}

// Is it possible to compose enums?
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
    MadeForYou,
}
