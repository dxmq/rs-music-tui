use anyhow::Error;
use futures::channel::oneshot::Sender;
use log::debug;
use reqwest::header::{
    HeaderMap, ACCEPT, ACCEPT_ENCODING, CACHE_CONTROL, PRAGMA, UPGRADE_INSECURE_REQUESTS,
    USER_AGENT,
};
use reqwest::Method;
use std::io::prelude::*;
use tempfile::NamedTempFile;

#[tokio::main]
pub async fn fetch_data(url: &str, buffer: NamedTempFile, tx: Sender<String>) -> Result<(), Error> {
    let mut buffer = buffer;

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
    let builder = client.request(Method::GET, url).headers(headers);
    let mut res = builder.send().await?;

    debug!("start download");
    if let Some(chunk) = res.chunk().await? {
        debug!("first chunk");
        Write::write_all(&mut buffer, &chunk[..]).unwrap();
        // buffer.write(&chunk[..]).unwrap();
        send_msg(tx);
    }

    while let Some(chunk) = res.chunk().await? {
        // bytes
        // buffer.write(&chunk[..]).unwrap();
        Write::write_all(&mut buffer, &chunk[..]).unwrap();
    }
    debug!("finish downloa");
    Ok(())
}

fn send_msg(tx: Sender<String>) {
    tx.send("ok".to_owned()).expect("send error");
}
