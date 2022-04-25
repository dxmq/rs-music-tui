use crate::cli::clap::BANNER;
use crate::http::api::CloudMusicApi;
use anyhow::anyhow;
use anyhow::Result;
use std::fs;
use std::io::stdin;
use std::path::{Path, PathBuf};

pub(crate) mod api;
mod client;
mod crypto;
mod key;
mod request;
mod response;
mod route;
mod store;

const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "netease-cloud-music-tui";
const COOKIE_FILE_NAME: &str = "cookie";

pub async fn login_phone() -> Result<()> {
    let cookie_path = get_or_build_cookie_paths()?;
    if !cookie_path.exists() {
        println!("{}", BANNER);
        println!(" Login In Netease Cloud Music Tui.");
        const MAX_RETRIES: u8 = 5;
        let mut num_retries = 0;
        loop {
            let phone = receive_key_from_input("phone")?;
            let password = receive_key_from_input("password")?;
            let resp = CloudMusicApi::default()
                .login_phone(&phone, &password)
                .await;
            if resp.is_err() {
                println!("登录失败……");
                println!("{}", "-".repeat(20));
                num_retries += 1;
                if num_retries == MAX_RETRIES {
                    // return Err(Error::from(std::io::Error::new(
                    //     std::io::ErrorKind::Other,
                    //     format!("Maximum retries ({}) exceeded.", MAX_RETRIES),
                    // )));
                    panic!("Maximum retries ({}) exceeded.", MAX_RETRIES)
                }
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

fn get_or_build_cookie_paths() -> Result<PathBuf> {
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
