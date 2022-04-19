#[macro_use]
extern crate lazy_static;

use crate::api::ApiEvent;
use anyhow::Result;
use ncmapi::NcmApi;
use std::sync::mpsc;

use crate::cli::clap::ClapApplication;
use crate::http::request::RequestClient;

mod api;
mod cli;
mod http;
mod ui;
mod util;

// #[tokio::main]
// async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//     let api = NcmApi::default();
//     let resp = api.cloud_search("mota", None).await;
//     let res = resp.unwrap().deserialize_to_implict();
//     println!("{:#?}", res);
//
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<()> {
    let mut clap_app = ClapApplication::new();
    let matches = clap_app.app.clone().get_matches();
    if let Some(s) = matches.value_of("completions") {
        return clap_app.gen_completions(s);
    }

    let (sync_io_tx, sync_io_rx) = mpsc::channel::<ApiEvent>();
    Ok(())
}
