use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{anyhow, Result};

use crate::event::Key;
use crate::ui::theme::Theme;

const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "netease-cloud-music-tui";
const CONFIG_FILE_NAME: &str = "config.yml";
const COOKIE_FILE_NAME: &str = "cookie.txt";

#[derive(Debug, Default)]
pub struct CookieConfig {
    pub cache: bool,
    pub cache_exp: Duration,
    pub cache_clean_interval: Duration,

    pub preserve_cookies: bool,
    pub cookie_path: PathBuf,

    pub log_request: bool,
    pub log_response: bool,
}

#[derive(Clone)]
pub struct UserConfig {
    pub path_to_config: Option<UserConfigPath>,
    pub behavior: BehaviorConfig,
    pub theme: Theme,
    pub keys: KeyBindings,
}

#[derive(Clone)]
pub struct KeyBindings {
    pub back: Key,
    pub next_page: Key,
    pub previous_page: Key,
    pub jump_to_start: Key,
    pub jump_to_end: Key,
    pub jump_to_album: Key,
    pub jump_to_artist_album: Key,
    pub jump_to_context: Key,
    pub manage_devices: Key,
    pub decrease_volume: Key,
    pub increase_volume: Key,
    pub toggle_playback: Key,
    pub seek_backwards: Key,
    pub seek_forwards: Key,
    pub next_track: Key,
    pub previous_track: Key,
    pub help: Key,
    pub shuffle: Key,
    pub repeat: Key,
    pub search: Key,
    pub submit: Key,
    pub copy_song_url: Key,
    pub copy_album_url: Key,
    pub audio_analysis: Key,
    pub basic_view: Key,
    pub add_item_to_queue: Key,
}

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

#[derive(Clone)]
pub struct UserConfigPath {
    pub config_file_path: PathBuf,
}

impl CookieConfig {
    fn default() -> Self {
        CookieConfig {
            cache: true,
            cache_exp: Duration::from_secs(3 * 60),
            cache_clean_interval: Duration::from_secs(6 * 60),
            preserve_cookies: true,
            cookie_path: UserConfig::cookie_path().unwrap(),
            log_request: false,
            log_response: false,
        }
    }
}

impl UserConfig {
    pub fn new() -> Self {
        UserConfig {
            path_to_config: None,
            behavior: BehaviorConfig::default(),
            theme: Default::default(),
            keys: KeyBindings {
                back: Key::Char('q'),
                next_page: Key::Ctrl('d'),
                previous_page: Key::Ctrl('u'),
                jump_to_start: Key::Ctrl('a'),
                jump_to_end: Key::Ctrl('e'),
                jump_to_album: Key::Char('a'),
                jump_to_artist_album: Key::Char('A'),
                jump_to_context: Key::Char('o'),
                manage_devices: Key::Char('d'),
                decrease_volume: Key::Char('-'),
                increase_volume: Key::Char('+'),
                toggle_playback: Key::Char(' '),
                seek_backwards: Key::Char('<'),
                seek_forwards: Key::Char('>'),
                next_track: Key::Char('n'),
                previous_track: Key::Char('p'),
                help: Key::Char('?'),
                shuffle: Key::Ctrl('s'),
                repeat: Key::Ctrl('r'),
                search: Key::Char('/'),
                submit: Key::Enter,
                copy_song_url: Key::Char('c'),
                copy_album_url: Key::Char('C'),
                audio_analysis: Key::Char('v'),
                basic_view: Key::Char('B'),
                add_item_to_queue: Key::Char('z'),
            },
        }
    }

    pub fn config_path(&mut self) -> Result<()> {
        let app_config_dir = UserConfig::build_app_config_dir()?;
        let config_file_path = &app_config_dir.join(CONFIG_FILE_NAME);

        let paths = UserConfigPath {
            config_file_path: config_file_path.to_path_buf(),
        };
        self.path_to_config = Some(paths);
        Ok(())
    }

    fn cookie_path() -> Result<PathBuf> {
        let app_config_dir = UserConfig::build_app_config_dir()?;
        Ok(app_config_dir.join(COOKIE_FILE_NAME))
    }

    pub fn build_app_config_dir() -> Result<PathBuf> {
        match dirs::home_dir() {
            Some(home) => {
                let path = Path::new(&home);
                let home_config_dir = path.join(CONFIG_DIR);
                let app_config_dir = home_config_dir.join(APP_CONFIG_DIR);

                if !home_config_dir.exists() {
                    fs::create_dir(&home_config_dir)?;
                }

                if !app_config_dir.exists() {
                    fs::create_dir(&app_config_dir)?;
                }

                Ok(app_config_dir)
            }
            None => Err(anyhow!("No $HOME directory found for client config")),
        }
    }

    pub fn padded_liked_icon(&self) -> String {
        format!("{} ", &self.behavior.liked_icon)
    }
}
