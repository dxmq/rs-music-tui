#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;
use std::sync::{mpsc, Arc};
use std::thread;

use anyhow::Result;
use tokio::sync::Mutex;

use crate::app::App;
use crate::cli::clap::ClapApplication;
use crate::config::user_config::{UserConfig, UserConfigPath};
use crate::event::IoEvent;
use crate::network::network::Network;

// mod api;
mod app;
mod cli;
mod config;
mod event;
mod handlers;
mod http;
mod model;
mod network;
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
    user_config.load_config()?;
    let (sync_io_tx, sync_io_rx) = mpsc::channel::<IoEvent>();
    let app: Arc<Mutex<App>> = Arc::new(Mutex::new(App::new(sync_io_tx, user_config.clone())));
    let clone_app = app.clone();
    thread::spawn(move || {
        let mut network = Network::new(&app);
        network::start_tokio(sync_io_rx, &mut network);
    });
    ui::start_ui(user_config, &clone_app).await?;
    Ok(())
}
