#[cfg(test)]
mod tests {
    use anyhow::Error;
    use futures::channel::oneshot;
    use futures::channel::oneshot::Sender;
    use log::debug;
    use reqwest::header::{
        HeaderMap, ACCEPT, ACCEPT_ENCODING, CACHE_CONTROL, PRAGMA, UPGRADE_INSECURE_REQUESTS,
        USER_AGENT,
    };
    use reqwest::Method;
    use rodio::source::SineWave;
    use rodio::{Decoder, OutputStream, Sink, Source};
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::time::Duration;
    use std::{thread, time};
    use tempfile::NamedTempFile;

    #[tokio::main]
    pub async fn fetch_data(
        url: &str,
        buffer: NamedTempFile,
        tx: Sender<String>,
    ) -> Result<(), Error> {
        // debug!("start fetch_data");
        let mut buffer = buffer;

        let mut headers = HeaderMap::new();
        headers.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
        headers.insert(PRAGMA, "no-cache".parse().unwrap());
        headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());

        headers.insert(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3".parse().unwrap());
        headers.insert(ACCEPT_ENCODING, "gzip,deflate".parse().unwrap());
        headers.insert(
            USER_AGENT,
            "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/13.10586"
                .parse()
                .unwrap(),
        );
        let client = reqwest::Client::builder()
            // no need proxy but can add it in config
            // .proxy(reqwest::Proxy::all("socks5://127.0.0.1:3333").expect("proxy error"))
            .build()
            .expect("builder error");
        let builder = client.request(Method::GET, url).headers(headers);
        let mut res = builder.send().await?;

        debug!("start download");
        if let Some(chunk) = res.chunk().await? {
            debug!("first chunk");
            buffer.write(&chunk[..]).unwrap();
            send_msg(tx);
        }

        while let Some(chunk) = res.chunk().await? {
            // bytes
            buffer.write(&chunk[..]).unwrap();
        }
        debug!("finish downloa");
        Ok(())
    }

    fn send_msg(tx: Sender<String>) {
        tx.send("ok".to_owned()).expect("send error");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test() {
        let url = String::from("http://m7.music.126.net/20220424114015/a561b5f63cedb0eabc68c40bdf39b089/ymusic/obj/w5zDlMODwrDDiGjCn8Ky/13928074750/e4a2/e072/cc54/b791036bfb269eba9ab3424afe93f4e6.mp3");
        let buffer = NamedTempFile::new().unwrap();
        let path = buffer.path().to_string_lossy().to_string();

        // debug!("start fetch_data");
        let mut buffer = buffer;
        let path = "D:/logs/aaa.mp3";
        // let file = std::fs::File::options().append(true).open(path).unwrap();
        println!("{}", path);

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let f = std::fs::File::open(&path).unwrap();
        let source = rodio::Decoder::new(std::io::BufReader::new(f)).unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        sink.play();
        sink.append(source);
        // let (ptx, mut prx) = oneshot::channel::<String>();
        //
        // thread::spawn(move || {
        //     fetch_data(&url.to_owned(), buffer, ptx).expect("error thread task");
        // });
        // loop {
        //     match prx.try_recv() {
        //         Ok(p) => match p {
        //             Some(_) => {
        //                 load_track(path.clone());
        //             }
        //             None => {}
        //         },
        //         Err(_) => {}
        //     }
        //     let t = time::Duration::from_millis(250);
        //     thread::sleep(t);
        // }
    }

    fn load_track(file: String) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let f = std::fs::File::open(&file).unwrap();
        let source = rodio::Decoder::new(std::io::BufReader::new(f)).unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        sink.play();
        sink.append(source);
    }

    pub fn load(file: String) -> Result<String, Error> {
        let f = std::fs::File::open(&file).unwrap();
        let source = Decoder::new(std::io::BufReader::new(f)).unwrap();
        let duration = match source.total_duration() {
            Some(d) => d,
            None => mp3_duration::from_path(&file).unwrap(),
        };
        // Ok(d) => d,
        // Err(_) => std::time::Duration::new(100, 0)
        // };
        Ok(String::from(file))
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_play() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let file = "D:/logs/aaa.mp3";
        // Add a dummy source of the sake of the example.
        let f = std::fs::File::open(&file).unwrap();
        let source = rodio::Decoder::new(std::io::BufReader::new(f)).unwrap();

        sink.append(source);

        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
        sink.sleep_until_end();
    }

    #[test]
    fn test_create_temp_file() {
        let mut file1 = NamedTempFile::new().unwrap();
        let path = file1.path().to_string_lossy().to_string();
        let text = "Brian was here. Briefly.";
        file1.write_all(text.as_bytes());
        println!("path: {}", path);
    }
}
