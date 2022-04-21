use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tui::style::Color;

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

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserConfigString {
    keybindings: Option<KeyBindingsString>,
    behavior: Option<BehaviorConfigString>,
    theme: Option<UserTheme>,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyBindingsString {
    back: Option<String>,
    next_page: Option<String>,
    previous_page: Option<String>,
    jump_to_start: Option<String>,
    jump_to_end: Option<String>,
    jump_to_album: Option<String>,
    jump_to_artist_album: Option<String>,
    jump_to_context: Option<String>,
    manage_devices: Option<String>,
    decrease_volume: Option<String>,
    increase_volume: Option<String>,
    toggle_playback: Option<String>,
    seek_backwards: Option<String>,
    seek_forwards: Option<String>,
    next_track: Option<String>,
    previous_track: Option<String>,
    help: Option<String>,
    shuffle: Option<String>,
    repeat: Option<String>,
    search: Option<String>,
    submit: Option<String>,
    copy_song_url: Option<String>,
    copy_album_url: Option<String>,
    audio_analysis: Option<String>,
    basic_view: Option<String>,
    add_item_to_queue: Option<String>,
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UserTheme {
    pub active: Option<String>,
    pub banner: Option<String>,
    pub error_border: Option<String>,
    pub error_text: Option<String>,
    pub hint: Option<String>,
    pub hovered: Option<String>,
    pub inactive: Option<String>,
    pub playbar_background: Option<String>,
    pub playbar_progress: Option<String>,
    pub playbar_progress_text: Option<String>,
    pub playbar_text: Option<String>,
    pub selected: Option<String>,
    pub text: Option<String>,
    pub header: Option<String>,
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

    fn build_paths(&mut self) -> Result<()> {
        let app_config_dir = UserConfig::build_app_config_dir()?;
        let config_file_path = app_config_dir.join(CONFIG_FILE_NAME);
        let paths = UserConfigPath {
            config_file_path: config_file_path.to_path_buf(),
        };
        self.path_to_config = Some(paths);
        Ok(())
    }

    pub fn load_config(&mut self) -> Result<()> {
        let paths = match &self.path_to_config {
            Some(path) => path,
            None => {
                self.build_paths()?;
                self.path_to_config.as_ref().unwrap()
            }
        };

        if paths.config_file_path.exists() {
            let config_string = fs::read_to_string(&paths.config_file_path)?;
            if config_string.is_empty() {
                return Ok(());
            }

            let config_yml: UserConfigString = serde_yaml::from_str(&config_string)?;

            if let Some(keybindings) = config_yml.keybindings.clone() {
                self.load_keybindings(keybindings)?;
            }

            if let Some(behavior) = config_yml.behavior {
                self.load_behaviorconfig(behavior)?;
            }
            if let Some(theme) = config_yml.theme {
                self.load_theme(theme)?;
            }
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn load_keybindings(&mut self, keybindings: KeyBindingsString) -> Result<()> {
        macro_rules! to_keys {
            ($name: ident) => {
                if let Some(key_string) = keybindings.$name {
                    self.keys.$name = parse_key(key_string)?;
                    check_reserved_keys(self.keys.$name)?;
                }
            };
        }

        to_keys!(back);
        to_keys!(next_page);
        to_keys!(previous_page);
        to_keys!(jump_to_start);
        to_keys!(jump_to_end);
        to_keys!(jump_to_album);
        to_keys!(jump_to_artist_album);
        to_keys!(jump_to_context);
        to_keys!(manage_devices);
        to_keys!(decrease_volume);
        to_keys!(increase_volume);
        to_keys!(toggle_playback);
        to_keys!(seek_backwards);
        to_keys!(seek_forwards);
        to_keys!(next_track);
        to_keys!(previous_track);
        to_keys!(help);
        to_keys!(shuffle);
        to_keys!(repeat);
        to_keys!(search);
        to_keys!(submit);
        to_keys!(copy_song_url);
        to_keys!(copy_album_url);
        to_keys!(audio_analysis);
        to_keys!(basic_view);
        to_keys!(add_item_to_queue);

        Ok(())
    }

    pub fn load_theme(&mut self, theme: UserTheme) -> Result<()> {
        macro_rules! to_theme_item {
            ($name: ident) => {
                if let Some(theme_item) = theme.$name {
                    self.theme.$name = parse_theme_item(&theme_item)?;
                }
            };
        }

        to_theme_item!(active);
        to_theme_item!(banner);
        to_theme_item!(error_border);
        to_theme_item!(error_text);
        to_theme_item!(hint);
        to_theme_item!(hovered);
        to_theme_item!(inactive);
        to_theme_item!(playbar_background);
        to_theme_item!(playbar_progress);
        to_theme_item!(playbar_progress_text);
        to_theme_item!(playbar_text);
        to_theme_item!(selected);
        to_theme_item!(text);
        to_theme_item!(header);
        Ok(())
    }

    pub fn load_behaviorconfig(&mut self, behavior_config: BehaviorConfigString) -> Result<()> {
        if let Some(behavior_string) = behavior_config.seek_milliseconds {
            self.behavior.seek_milliseconds = behavior_string;
        }

        if let Some(behavior_string) = behavior_config.volume_increment {
            if behavior_string > 100 {
                return Err(anyhow!(
                    "Volume increment must be between 0 and 100, is {}",
                    behavior_string,
                ));
            }
            self.behavior.volume_increment = behavior_string;
        }

        if let Some(tick_rate) = behavior_config.tick_rate_milliseconds {
            if tick_rate >= 1000 {
                return Err(anyhow!("Tick rate must be below 1000"));
            } else {
                self.behavior.tick_rate_milliseconds = tick_rate;
            }
        }

        if let Some(text_emphasis) = behavior_config.enable_text_emphasis {
            self.behavior.enable_text_emphasis = text_emphasis;
        }

        if let Some(loading_indicator) = behavior_config.show_loading_indicator {
            self.behavior.show_loading_indicator = loading_indicator;
        }

        if let Some(wide_search_bar) = behavior_config.enforce_wide_search_bar {
            self.behavior.enforce_wide_search_bar = wide_search_bar;
        }

        if let Some(liked_icon) = behavior_config.liked_icon {
            self.behavior.liked_icon = liked_icon;
        }

        if let Some(paused_icon) = behavior_config.paused_icon {
            self.behavior.paused_icon = paused_icon;
        }

        if let Some(playing_icon) = behavior_config.playing_icon {
            self.behavior.playing_icon = playing_icon;
        }

        if let Some(shuffle_icon) = behavior_config.shuffle_icon {
            self.behavior.shuffle_icon = shuffle_icon;
        }

        if let Some(repeat_track_icon) = behavior_config.repeat_track_icon {
            self.behavior.repeat_track_icon = repeat_track_icon;
        }

        if let Some(repeat_context_icon) = behavior_config.repeat_context_icon {
            self.behavior.repeat_context_icon = repeat_context_icon;
        }

        if let Some(set_window_title) = behavior_config.set_window_title {
            self.behavior.set_window_title = set_window_title;
        }

        Ok(())
    }

    pub fn padded_liked_icon(&self) -> String {
        format!("{} ", &self.behavior.liked_icon)
    }
}

fn check_reserved_keys(key: Key) -> Result<()> {
    let reserved = [
        Key::Char('h'),
        Key::Char('j'),
        Key::Char('k'),
        Key::Char('l'),
        Key::Char('H'),
        Key::Char('M'),
        Key::Char('L'),
        Key::Up,
        Key::Down,
        Key::Left,
        Key::Right,
        Key::Backspace,
        Key::Enter,
    ];
    for item in reserved.iter() {
        if key == *item {
            // TODO1: Add pretty print for key
            return Err(anyhow!(
                "The key {:?} is reserved and cannot be remapped",
                key
            ));
        }
    }
    Ok(())
}

fn parse_key(key: String) -> Result<Key> {
    fn get_single_char(string: &str) -> char {
        match string.chars().next() {
            Some(c) => c,
            None => panic!(),
        }
    }

    match key.len() {
        1 => Ok(Key::Char(get_single_char(key.as_str()))),
        _ => {
            let sections: Vec<&str> = key.split('-').collect();

            if sections.len() > 2 {
                return Err(anyhow!(
                    "Shortcut can only have 2 keys, \"{}\" has {}",
                    key,
                    sections.len()
                ));
            }

            match sections[0].to_lowercase().as_str() {
                "ctrl" => Ok(Key::Ctrl(get_single_char(sections[1]))),
                "alt" => Ok(Key::Alt(get_single_char(sections[1]))),
                "left" => Ok(Key::Left),
                "right" => Ok(Key::Right),
                "up" => Ok(Key::Up),
                "down" => Ok(Key::Down),
                "backspace" | "delete" => Ok(Key::Backspace),
                "del" => Ok(Key::Delete),
                "esc" | "escape" => Ok(Key::Esc),
                "pageup" => Ok(Key::PageUp),
                "pagedown" => Ok(Key::PageDown),
                "space" => Ok(Key::Char(' ')),
                _ => Err(anyhow!("The key \"{}\" is unknown.", sections[0])),
            }
        }
    }
}

fn parse_theme_item(theme_item: &str) -> Result<Color> {
    let color = match theme_item {
        "Reset" => Color::Reset,
        "Black" => Color::Black,
        "Red" => Color::Red,
        "Green" => Color::Green,
        "Yellow" => Color::Yellow,
        "Blue" => Color::Blue,
        "Magenta" => Color::Magenta,
        "Cyan" => Color::Cyan,
        "Gray" => Color::Gray,
        "DarkGray" => Color::DarkGray,
        "LightRed" => Color::LightRed,
        "LightGreen" => Color::LightGreen,
        "LightYellow" => Color::LightYellow,
        "LightBlue" => Color::LightBlue,
        "LightMagenta" => Color::LightMagenta,
        "LightCyan" => Color::LightCyan,
        "White" => Color::White,
        _ => {
            let colors = theme_item.split(',').collect::<Vec<&str>>();
            if let (Some(r), Some(g), Some(b)) = (colors.get(0), colors.get(1), colors.get(2)) {
                Color::Rgb(
                    r.trim().parse::<u8>()?,
                    g.trim().parse::<u8>()?,
                    b.trim().parse::<u8>()?,
                )
            } else {
                println!("Unexpected color {}", theme_item);
                Color::Black
            }
        }
    };

    Ok(color)
}
