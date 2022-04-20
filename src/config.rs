use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::ui::theme::Theme;
use anyhow::{anyhow, Result};

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

#[derive(Clone, Default)]
pub struct UserConfig {
    pub path_to_config: Option<UserConfigPath>,
    pub behavior: BehaviorConfig,
    pub theme: Theme,
}

#[derive(Clone)]
pub struct BehaviorConfig {
    pub tick_rate_milliseconds: u64,
    pub set_window_title: bool,
    // 是否强制执行宽搜索栏
    pub enforce_wide_search_bar: bool,
    // 是否展示加载指示器
    pub show_loading_indicator: bool,
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            tick_rate_milliseconds: 250,
            set_window_title: true,
            enforce_wide_search_bar: false,
            show_loading_indicator: true,
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
}
