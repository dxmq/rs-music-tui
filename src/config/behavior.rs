use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct BehaviorConfig {
    // 快进毫秒数
    pub seek_milliseconds: u32,
    // 声音增加数
    pub volume_increment: u8,
    pub tick_rate_milliseconds: u64,
    pub set_window_title: bool,
    // 是否强制执行宽搜索栏
    pub enforce_wide_search_bar: bool,
    // 是否展示加载指示器
    pub show_loading_indicator: bool,
    // 收藏图标
    pub liked_icon: String,
    // 随机播放图标
    pub shuffle_icon: String,
    // 单曲循环播放图标
    pub repeat_track_icon: String,
    // 列表循环播放图标
    pub repeat_context_icon: String,
    // 播放图标
    pub playing_icon: String,
    // 暂停图标
    pub paused_icon: String,
    // 是否开启字体强调
    pub enable_text_emphasis: bool,
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            seek_milliseconds: 5 * 1000,
            volume_increment: 10,
            tick_rate_milliseconds: 250,
            set_window_title: true,
            enforce_wide_search_bar: false,
            show_loading_indicator: true,
            liked_icon: "♥".to_string(),
            shuffle_icon: "🔀".to_string(),
            repeat_track_icon: "🔂".to_string(),
            repeat_context_icon: "🔁".to_string(),
            playing_icon: "▶".to_string(),
            paused_icon: "⏸".to_string(),
            enable_text_emphasis: true,
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BehaviorConfigString {
    pub seek_milliseconds: Option<u32>,
    pub volume_increment: Option<u8>,
    pub tick_rate_milliseconds: Option<u64>,
    pub enable_text_emphasis: Option<bool>,
    pub show_loading_indicator: Option<bool>,
    pub enforce_wide_search_bar: Option<bool>,
    pub liked_icon: Option<String>,
    pub shuffle_icon: Option<String>,
    pub repeat_track_icon: Option<String>,
    pub repeat_context_icon: Option<String>,
    pub playing_icon: Option<String>,
    pub paused_icon: Option<String>,
    pub set_window_title: Option<bool>,
}
