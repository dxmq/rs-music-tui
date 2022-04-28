use std::io::prelude::*;

use anyhow::Error;
use futures::channel::oneshot::Sender;
use reqwest::header::{
    HeaderMap, ACCEPT, ACCEPT_ENCODING, CACHE_CONTROL, PRAGMA, UPGRADE_INSECURE_REQUESTS,
    USER_AGENT,
};
use reqwest::Method;
use tempfile::NamedTempFile;

#[tokio::main]
pub async fn fetch_data(url: &str, tx: Sender<String>) -> Result<(), Error> {
    // let mut buffer = buffer;
    let mut headers = HeaderMap::new();
    headers.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
    headers.insert(PRAGMA, "no-cache".parse().unwrap());
    headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());
    headers.insert(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip,deflate".parse().unwrap());
    headers.insert(
        USER_AGENT,
        "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/13.10586".parse().unwrap(),
    );
    let client = reqwest::Client::builder().build().expect("builder error");
    let mut res = client
        .request(Method::GET, url)
        .headers(headers)
        .send()
        .await?;
    let mut file = NamedTempFile::new()?;

    while let Some(chunk) = res.chunk().await? {
        Write::write_all(&mut file, &chunk[..]).unwrap();
    }
    let path = file.into_temp_path();
    let path = path.keep()?;
    let file_path = path.to_string_lossy().to_string();
    send_msg(tx, file_path.as_str());
    Ok(())
}

fn send_msg(tx: Sender<String>, filename: &str) {
    tx.send(filename.to_owned()).expect("send error");
}
