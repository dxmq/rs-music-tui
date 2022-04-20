#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;
use std::sync::{mpsc, Arc, Mutex};

use anyhow::Result;

use crate::api::ApiEvent;
use crate::app::App;
use crate::cli::clap::ClapApplication;
use crate::config::{UserConfig, UserConfigPath};

mod api;
mod app;
mod cli;
mod config;
mod event;
mod http;
mod ui;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    let mut clap_app = ClapApplication::new();
    let matches = clap_app.app.clone().get_matches();
    if let Some(s) = matches.value_of("completions") {
        return clap_app.gen_completions(s);
    }

    let mut user_config = UserConfig::new();
    if let Some(config_file_path) = matches.value_of("config") {
        let config_file_path = PathBuf::from(config_file_path);
        let config_path = UserConfigPath { config_file_path };
        user_config.path_to_config.replace(config_path);
    }
    let (sync_io_tx, sync_io_rx) = mpsc::channel::<ApiEvent>();
    let app: Arc<Mutex<App>> = Arc::new(Mutex::new(App::new(sync_io_tx, user_config)));
    Ok(())
}

// #[tokio::test(flavor = "multi_thread")]
// async fn test_user_subcount() {
//     let api = NcmApi::default();
//     let resp = api.user_subcount().await;
//     assert!(resp.is_ok());
//
//     let res = resp.unwrap();
//     let res = res.deserialize_to_implict();
//     assert_eq!(res.code, 200);
// }
