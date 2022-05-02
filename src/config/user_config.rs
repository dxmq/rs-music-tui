use std::fs;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tui::style::Color;

use crate::config::behavior::{BehaviorConfig, BehaviorConfigString};
use crate::config::keybinds::{KeyBindings, KeyBindingsString};
use crate::config::theme::{Theme, UserTheme};
use crate::event::Key;

const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "netease-cloud-music-tui";
const CONFIG_FILE_NAME: &str = "config.yml";
const CACHE_FILE_NAME: &str = "cache.json";

#[derive(Clone)]
pub struct UserConfig {
    pub path_to_config: Option<UserConfigPath>,
    pub behavior: BehaviorConfig,
    pub theme: Theme,
    pub keys: KeyBindings,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserConfigString {
    keybindings: Option<KeyBindingsString>,
    behavior: Option<BehaviorConfigString>,
    theme: Option<UserTheme>,
}

#[derive(Clone)]
pub struct UserConfigPath {
    pub config_file_path: PathBuf,
    pub cache_file_path: PathBuf,
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
                increase_volume: Key::Char('='),
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
                show_lyric: Key::Ctrl('l'),
            },
        }
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
        let cache_file_path = app_config_dir.join(CACHE_FILE_NAME);
        let paths = UserConfigPath {
            config_file_path,
            cache_file_path,
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
