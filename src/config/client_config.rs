use std::fs;
use std::io::stdin;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{anyhow, Error, Result};
use serde::{Deserialize, Serialize};

use crate::cli::clap::BANNER;
use crate::config::user_config::UserConfig;
use crate::network::api;

const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "netease-cloud-music-tui";
const COOKIE_FILE_NAME: &str = "cookie.txt";

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ClientConfig {
    pub cache: bool,
    pub cache_exp: Duration,
    pub cache_clean_interval: Duration,

    pub preserve_cookies: bool,
    pub cookie_path: PathBuf,
    pub log_request: bool,
    pub log_response: bool,
}

pub struct ConfigPaths {
    // cookie配置文件路径
    pub cookie_file_path: PathBuf,
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig {
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

impl ClientConfig {
    pub async fn load_config(&mut self) -> Result<()> {
        let cookie_file_path = self.get_or_build_paths()?;
        if !cookie_file_path.exists() {
            println!("{}", BANNER);
            println!(" Login In Netease Cloud Music Tui.");
            loop {
                let phone = ClientConfig::receive_key_from_input("phone")?;
                let password = ClientConfig::receive_key_from_input("password")?;
                let resp = api().login_phone(&phone, &password).await;
                if !resp.is_ok() {
                    println!("{}", "登录失败……");
                    println!("{}", "-".repeat(20));
                } else {
                    break;
                }
            }
        }
        Ok(())
    }

    fn receive_key_from_input(type_label: &'static str) -> Result<String> {
        let mut input = String::new();
        println!("\nEnter your {}: ", type_label);
        // 读取输入字符
        stdin().read_line(&mut input)?;
        // 去左右空格
        input = input.trim().to_string();
        Ok(input)
    }

    pub fn get_or_build_paths(&self) -> Result<PathBuf> {
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

                let cookie_file_path = &app_config_dir.join(COOKIE_FILE_NAME);

                let paths = cookie_file_path.to_path_buf();
                Ok(paths)
            }
            None => Err(anyhow!("No $HOME directory found for client config")),
        }
    }
}
