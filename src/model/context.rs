use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::device::Device;
use crate::model::enums::{CurrentlyPlayingType, DisallowKey, PlayingItem, RepeatState, Type};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Context {
    pub uri: String,
    pub href: String,
    pub external_urls: HashMap<String, String>,
    pub _type: Type,
}

// 当前正在播放上下文
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CurrentlyPlayingContext {
    // 播放状态
    pub is_playing: bool,
    // 当前进度（毫秒）
    pub process_ms: Option<u32>,
    pub timestamp: u64,
    pub context: Option<Context>,
}

// 当前回放上下文
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CurrentlyPlaybackContext {
    // 播放状态
    pub is_playing: bool,
    // 当前进度（毫秒）
    pub process_ms: Option<u32>,
    pub timestamp: u64,
    pub context: Option<Context>,
    // 当前播放的是什么
    pub currently_playing_type: CurrentlyPlayingType,
    // 动作
    pub active: Action,
    // 重复状态
    pub repeat_state: RepeatState,
    // 是否随机播放
    pub shuffle_state: bool,
    // 当前播放项
    pub item: Option<PlayingItem>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Action {
    pub disallows: HashMap<DisallowKey, bool>,
}

// 对话框
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DialogContext {
    PlaylistWindow,
    PlaylistSearch,
}

// Is it possible to compose enums?
#[derive(PartialEq, Debug)]
pub enum TrackTableContext {
    MyPlaylists,
    AlbumSearch,
    PlaylistSearch,
    SavedTracks,
    RecommendedTracks,
    MadeForYou,
}
