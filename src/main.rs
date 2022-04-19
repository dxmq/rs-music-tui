#[macro_use]
extern crate lazy_static;

use ncmapi::NcmApi;

use crate::http::request::RequestClient;

mod cli;
mod http;
mod ui;
mod util;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let api = NcmApi::default();
    let resp = api.cloud_search("mota", None).await;
    let res = resp.unwrap().deserialize_to_implict();
    println!("{:#?}", res);

    Ok(())
}
